---
title: "Secure NFS"
date: 2025-06-11
draft: false
---

Nfs is insecure by default. The host can filter on client IP but IP is easy to
spoof. Therefore (normal) nfs is insecure. You can use nfs with kerberos
authentication and authorization. 

A good overview of the kerberos algorithm is:
[Taming Kerberos - Computerphile](https://www.youtube.com/watch?v=qW361k3-BtU)

A reasonable guide to setting up nfs with kerberos:
[Network File System (NFS) - Ubuntu](https://documentation.ubuntu.com/server/how-to/networking/install-nfs/#nfs-with-kerberos)

Most guides, include the Ubuntu one above assume you are a sys admin. You and I
are not. So lets go through some of things not mentioned.

# Things not mentioned
## You need domain names
Good news though, they need not be global. No need to buy anything. I assume nfs and
kerberos need a name to refer to a server. You might get away with using the
IP adresses.

A simple way to do this is to add each machines hostname and IP to the hosts
file in `/etc/hosts`. I have a file server named *asgard*, it has local IP
192.168.1.15. Therefore I have this in the hosts file of my workstation: 
```hosts
192.168.1.15 	asgard
```

## You must run kadmin as sudo
At least when export keys, if you do not you will get the error `Permission denied while adding key to keytab` or `Key table file '/etc/krb5.keytab' not found while adding key to keytab`. That last one I could not find explained anywhere, lost quite some time on that. These errors occur because without sudo the keytab (usually at /etc/krb5.keytab) can not be read or created.

## You must share ZFS datasets seperatly
Given the datasets:
```
/srv/
/srv/music
/srv/pictures
```
You can not share only `/srv`. The share will mount but users will get
`permission denied` on any file operation inside `music` & `pictures`.

# Full Procedure on the Server
This assumes you are building a basic file server such as a NAS:
- You have one server you want to run NFS on
- You do not have kerberos installed
- You have no other needs for kerberos
- All clients are on the same local network as the server

## Set up kerberos
This is based on [How to install a kerberos server - Ubuntu](https://documentation.ubuntu.com/server/how-to/kerberos/install-a-kerberos-server/) and my own findings. I recommend you read that in its enterity if you get stuck.

### Names
You need to decide on some names before we continue. Specifically you need a:
- Realm
- Host name for the server

In this guide I will be using the names I picked for my network (If you have not
guessed it yet this guide is mostly written for future me). My *Realm* is named: 
*Yggdrasil* and my file server is called *Asgard*.

Every client on my network gets `192.168.1.15    asgard` appended to their `hosts` file. 

### Setting up the Kerberos servers 
I am using debian 12 "bookworm", on debian based systems such as Ubuntu this
will all work. On other unix names you will have to find equivalent packages.

We start by installing three packages providing: 
- *kadmind*: a server handles account creation and deletion and other adminstrative commands.
- The user programs to interact with *kadmind*
- The *Key Distribution Center* (KDC). The server that handle creating tickets.
servers. The *kadmind* 

During the installation you should be promted to enter: 
- your realm name, enter it in all caps. I entered: `YGGDRASIL`. 
- Kerberos servers for your realm: This is where the KDC will run, in our case
  this server. I enter the host name for the server, in my case: `asgard`.
- Administrative server for your Kerberos realm: This is where we are 
  running *kadmind*, in our case this server. In my case again: `argard`.

```
$SERVER: sudo apt install krb5-kdc krb5-admin-server
```

If you did not get prompted then there are configuration files from a previous
installation present. You need to remove those first. Do so with:
```
$SERVER: sudo apt purge -y krb5-kdc krb5-admin-server krb5-config krb5-locales krb5-user
```
Then try the install again. If that still does not prompt you will have to
search for left over kerberos config files. [relevant stack exchange issue](https://serverfault.com/questions/592893/completely-uninstall-kerberos-on-ubuntu-server)

#### Creating the KDC database
Before we can use the KDC we need to create a database for it. You will
be prompted for a database password, store it somewhere securely. I store mine
in my personal password manager under the name `kerberos_database` (now you know
where to look hackers!).

```
$SERVER: sudo kdb5_util create -r YGGDRASIL -s
```

If you forget to create the database you will get the error: `kadmin.local: Cannot open DB2 database '/var/lib/krb5kdc/principal': No such file or directory while initializing kadmin.local interface`.

### Creating kerberos "users"
In kerberos users are called principles. There are two ways to add principles:
- as the root user on the machine where *kadmind* is running using `kadmin.local`.
- by logging in as a *kerberos admin* using the *kadmin* program.

#### Creating the kerberos admin
On each machine we want to be able to access the NFS server we need to create a
kerberos user. That user can only be created *on the client machine*. To do so
we need to log in to the adminstrative server (using *kadmin*) from the client
machine. For that we need a *kerberos admin*. 

We create an admin using kadmin.local (which only works on ther same machine as
the admin server and when run by sudo).

The name for this principle can be anything, I have chosen my *name* followed
by the word *admin*. You will be prompted to enter a password. If you lose that
you will have to make this user again.
```
$SERVER: sudo kadmin.local -q "addprinc david/admin"
```

We need to give this user the permissions that make it an admin. For that we
edit the the kerberos *Access Control List* (ACL) in `/etc/krb5kdc/kadm5.acl`.

Adapt the line below and add it to your ACL file.
```
*/admin@YGGDRASIL     *
```
I have used a star so any principle whoes name ends in */admin* gets all
permissions on the Kerberos servers.

Now restart the kerberos admin server using:
```
$SERVER: sudo systemctl restart krb5-admin-server
```

#### Creating the NFS principle
The name for the principle used by the NFS server must follow the format
<anything>/<host> or NFS will not with kerberos. The <host> must occur in every
clients hosts file. My server has the host name `asgard`.
```
$SERVER: sudo kadmin -p david/admin -q "addprinc -randkey nfs/asgard"
$SERVER: sudo kadmin -p david/admin -q "ktadd nfs/asgard"
```
Note: We could do this using `sudo kadmin.local` however by logging in as admin with `-p` we can catch any issues with the kerberos setup now. If you get the error: `kadmin: Communication failure with server while initializing kadmin interface` there is probably something wrong with one of the kerberos servers. Run `sudo systemctl list-units --failed` to figure out which service failed and then `sudo systemctl status <service_name>` to see what went wrong.

### Set up NFSv4
If you already have an NFS server you *might* need to remove it first. During install the package will start the required kerberos related NFS services. This does not happen if NFS is already installed. 

Install nfs with:
```
$SERVER: sudo apt install nfs-kernel-server
```

Now we just need to populate the /etc/exports file, I want to share the /srv
directory thus my exports file looks like this:
```
/srv *(rw,sync,no_subtree_check,sec=krb5)
```

Note: `krb5` makes nfs use kerberos only for authentication, a man in the middle
can intercept or inject nfs traffic. In my use case (a home network) the router
and switches are trusted therefore authentication is all I need. Alternatively
use `kerb5p` for encrypted & integrity verified NFS traffic.

Then refresh the export using:
```
$SERVER: sudo exportfs -rav
```
### Configure id mapping
File ownership & operations will be send with <name>@<realm> instead of user and
group id's. Kerberos takes care of authenticating that <name> actually belongs
to the user.

The NFS server and client need to now how <name>@<realm> maps to local unix
users and groups. The server translates this using `idmapd` based on the content
of `/etc/idmapd.conf`. 
Most guides describe how to couple idmap to a large organizations user database.
Since we are building a simple NAS we will use a fully menual (static)
configuration. For each kerberos principle we describe in `idmapd.conf` which
local user it corrosponds to.

This is a basic `idmapd.conf` for the realm `YGGDRASIL`. Actions performed and files owned by Kerberos principle `david` should be translated to actions performed and files owned by local UNIX user `david`.
```
[General]
Verbosity = 0
# set your own domain here, if it differs from FQDN minus hostname
Domain = yggdrasil

[Mapping]
Nobody-User = nobody
Nobody-Group = nogroup

[Translation]
Method = static
GSS-methods = static

[Static]
david/admin@YGGDRASIL = root
david@YGGDRASIL = david
```

# Procedure for each Client
## Add the server to hosts
To set up Kerberos and NFS on the client the client needs to know the IP for the
server. The server is referred to by its hostname. We make sure the client knows
it by entering it in the clients hosts file.

In my case I append `192.168.1.15    asgard` to the *hosts* file at `/etc/hosts`. 

## Install kerberos programs
Install the kerberos user programs to interact with *kadmind*. You should be
prompted for the same kerberos information as when you set up the server. That
is:
- The *default kerberos realm* 
- Kerberos servers for your realm.
- Administrative servers for your Kerberos realm.

For the *Kerberos servers* & *adminstrative servers* enter the host name you
just added to the *hosts file*.

```
$CLIENT: sudo apt install krb5-user
```

If you did not get prompted you probably have exisitng configuration. You can
try and remove that using `sudo apt purge krb5-user krb5-config`.

## Add Kerberos user for NFS client
We need two or more kerberos principle for each client. 
- One for the NFS client software, it must be named: `nfs/<host>` where host is
  the hostname for the client machine. Check what it is using `hostname`.
- One for each user on the machine that needs to access the share. It is easiest
  if it has the same name as the local user who is going to use nfs. Its okay to 
  share one between multiple clients machines.

We will use the kerberos admin server from the client. The advantage to that is
that we can use `ktadd` on the client and the key will then appear in the
clients local *keytab*. We use the kerberos admin server remotly by passing `-p
<admin user>` to `kadmin` instead of using `kadmin.local` with `sudo`. We still 
pass the command to execute to `kadmin` using `-q`.

```
$CLIENT: sudo kadmin -p david/admin -q "addprinc -randkey nfs/<host>" 
$CLIENT: sudo kadmin -p david/admin -q "ktadd nfs/<host>" 
$CLIENT: sudo kadmin -p david/admin -q "addprinc <local username>" 
$CLIENT: sudo kadmin -p david/admin -q "ktadd <local username>" 
```

## Install the NFS client & Mount
We need to do this *after* installing and setting up Kerberos on the client or
the extra kerberos services used for NFS will not be enabled and started.
```
$CLIENT: sudo apt install nfs-common
```

## Configure id mapping
Just like on the server we have to define a mapping between local users and
Kerberos users. The NFS client uses `nfsidmap` instead of `idmapd` however they
both read she same config file. Copy the one you made for the server to
`/etc/idmapd.conf`.

## Testing the mount
Now you need to log in as Kerberos user using `kinit`.

I have ran into an issue where I get invalid password when trying `kinit`.
Running `sudo kadmin -p david/admin -q change_password <local username>` to set
the password again seems to resolve that.

Then simply mount the shared directory via:
```
$CLIENT: sudo mount asgard:/srv /mnt
```

If this fails you need to restart the client machine. I have no idea why but it
works :)

## Auto mount on login/boot
Currently we need to mount with sudo *and* get a kerberos ticket. To get a
kerberos ticket we have to type in a password It gets worse kerberos tickets expire (after around a day). Lets automate this by:
- storing a keytab for the users principle in its homedir and use a keytab for
  authentication instead of the password.
- set up a systemd serice to request the ticket and renew it periodically after
  the user logs in.
- add the mount to fstab so its always mounted on boot (though without a valid
  kerberos ticket you can not use it)

### Passwordless kerberos tickets
Figure out the KVNO:
```
$CLIENT: kinit <username>
$CLIENT: kvno <username>
$CLIENT: kdestroy -p <username>
```

Generate a keytab for the user
```
$CLIENT: ktutil
ktutil: addent -password -p <NAME> -k <KVNO> -e aes128-cts-hmac-sha1-96 -f
ktutil: wkt /home/<username>/.local/keytab
ktutil: q
```

This should not prompt for a password:
```
$CLIENT: kinit -kt ~/.local/keytab -p <principle name>
```

### Automate ticket request and renewal
For this we need the programs: `k5start` and `krenew` lets install them:
```
$CLIENT: sudo apt install kstart
```

Now we want to run `k5start` once the user logs followed by `krenew`. We use
systemd to set that up.

This will open up an editor where you can write the service:
```
$CLIENT: systemctl edit --user --force --full kerberos_mount.service
```

Enter the following:
```
[Unit]
Description=Initializes, caches and renews Kerberos ticket
After=default.target

[Service]
Type=exec
RemainAfterExit=yes
ExecStart=/usr/bin/k5start \
	# run in daemon mode, recheck ticket every  30 minutes \
	-K 30 \
	# authenticate using keytab rather then asking for a password \
	-f %h/.local/keytab \
	# store this file as ticket cache \
	-k /tmp/krb5cc_%U \
	# principle to get tickets for \
	%u

[Install]
WantedBy=default.target
```

Enable and start the service:
```
$CLIENT: systemctl enable --now --user kerberos_mount.service
```

### Mount on boot in fstab

Add something like this to fstab (use: `sudoedit /etc/fstab`): 
```
asgard:/srv/music  /home/david/Music          nfs4  nofail,rw,sec=krb5
```
This will mount the share `music` in `/srv/music` on the server named `asgard`
in the host file to the folder `/home/david/Music`. Do not forget the `nofail`
option or the system will not boot if the file server is down.

Verify fstab entry by:
- unmounting the share using `sudo umount Music` 
- using `sudo mount -a` witout rebooting to make sure its correct:
```
$CLIENT: sudo mount -a
```

# Appendix

Some specific setups I need.

## Mpd systemd & kerberos
I run *MPD* (music play deamon) as a seperate user (named `mpd` in group `mpd`).
The systemd service listed above will not work since the `mpd` user is never
logged in. Rather the *MPD* process is started as root and drops privilleges. 

Instead of starting the ticket renewel when the `mpd` user logs in (which it never
will) we just start it on boot.

The systemd service for ticket renewel is almost the same. The changes are:
- Make it run as user `mpd` in the group `mpd`
- Hard code the principle as `mpd`
- Hard code the ticket cache location as `/tmp/krb5cc_mpd`
- Use the keytab `/mpd_keytab` (the mpd user has no home and so no .local)

This results in the systemd service file:
```
[Unit]
Description=Initializes, caches and renews Kerberos ticket
After=default.target

[Service]
Type=exec
User=mpd
Group=mpd
RemainAfterExit=yes
ExecStart=/usr/bin/k5start \
	# run in daemon mode, recheck ticket every  30 minutes \
	-K 30 \
	# authenticate using keytab rather then asking for a password \
	-f /mpd_keytab \
	# store this file as ticket cache \
	-k /tmp/krb5cc_mpd \
	# principle to get tickets for \
	mpd

[Install]
WantedBy=default.target
```
I've named this service kerberos_mount_for_mpd. 

Lets make the *MPD* service require the systemd service above. That way *MPD* will
report an error if the `kerberos_mount_for_mpd` service fails. We do this by
editing the service using:
```
$CLIENT: sudo systemctl edit --full mpd
```

The we simple:
- Append ` kerberos_mount_for_mpd.service` to the `After=` line
- Add a `Requires=kerberos_mount_for_mpd.service` line below the `After` one

For the rest follow the normal client procedure.

## Permission denied on the client

- Is the user on the client in the same group on the file server as the directory?
- Is the user on the client mapped to the right user on the file server?
- Has the folder/file the correct owning group?
- Are the permissions set correctly for that group?

## Have new items assume the group of their parent dir

We need to set a *default ACL* on the parent dir using `setfacl --default`. This
must be done on the file server. Such an ACL will be applied to every new file
and directory regardless of who created them. *Default ACL's* are prefixed with
`default:` when displayed using `getfacl`.

Ususally we want every file or directory created inside the share to be owned by
the same group. We leave the owner alone that way we can still see who made the
item.

Use the command:
```
sudo setfacl --default --modify group:<GroupName>:rwx <share folder>
```

For example:
```
sudo setfacl --default --modify group:music:rwx /srv/music
```
Makes any newly created directory or folder readable, writable and
executable/explorable by any user in the *music* group.

<!-- Lets fix that. -->
<!---->
<!-- At this point you should have working and secure fs mounts. The permissions are -->
<!-- all or nothing at this point. While you can maybe get ACL's working I decided I -->
<!-- spend enough time on all this nonesens. -->

<!---->
<!-- In the next section we will look at more fine grained permissions. -->
<!---->
<!-- # NFSv4 access control lists -->
<!-- We can control which directories on the server a kerberos principle (user) can -->
<!-- access.  -->
<!---->
<!-- ``` -->
<!-- sudo apt-get install acl -->
<!-- <!-- sudo apt install nfs4-acl-tools --> -->
<!-- ``` -->
<!---->
<!-- <!-- https://fjordtek.com/categories/news/2021/kerberos-secured-network-file-shares-practical-guide-for-kerberized-nfsv4 --> -->
<!-- <!-- https://www.illumos.org/books/zfs-admin/acls.html --> -->
<!---->
<!-- <!-- TODO ID mapper --> -->
<!-- <!-- https://manpages.ubuntu.com/manpages/lunar/en/man8/nfsidmap.8.html --> -->
<!-- <!-- https://manpages.ubuntu.com/manpages/lunar/en/man8/idmapd.8.html --> -->
<!---->
