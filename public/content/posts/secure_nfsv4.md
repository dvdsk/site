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

And we are done! Onwards to the client!

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
$CLIENT sudo apt install krb5-user
```

If you did not get prompted you probably have exisitng configuration. You can
try and remove that using `sudo apt purge krb5-user krb5-config`.

## Add Kerberos user for NFS client
Add a kerberos principle for the NFS client. For this we need to log in to the
kerberos admin server from the client. We do so by passing `-p <admin user>` to `kadmin` instead of using `kadmin.local` with `sudo`. Then we pass the command to execute to `kadmin` using `-q`.

```
$CLIENT sudo kadmin -p david/admin -q "addprinc -randkey nfs/dmain" 
$CLIENT sudo kadmin -p david/admin -q "ktadd nfs/dmain" 
```

## Install the NFS client & Mount
We need to do this *after* installing and setting up Kerberos on the client or
the extra kerberos services used for NFS will not be enabled and started.
```
$CLIENT sudo apt install nfs-common
```

Then simply mount the shared directory via:
```
$CLIENT sudo mount asgard:/srv /mnt
```

If this fails you need to restart the client machine. I have no idea why but it
works :)

At this point you should have working and securen fs mounts.
