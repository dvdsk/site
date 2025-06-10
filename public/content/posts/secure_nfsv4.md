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

### Setting up the Realm
I am using debian 12 "bookworm", on debian based systems such as Ubuntu this
will all work. On other unix names you will have to find equivalent packages.

We start by installing *kadmind* and the Key Distribution Center (KDC). Both are
servers. The *kadmind* server handles account creation and deletion and other
adminstrative commands.

This will prompt you to enter: 
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

<!-- ### Set Realm -->
<!-- Now we configure the Realm. This will prompt you for a master key, store it -->
<!-- somewhere securely. I store it in my password manager under the name -->
<!-- `kerberos_database` (good luck hackers!). -->
<!-- ``` -->
<!-- $SERVER: sudo krb5_newrealm -->
<!-- ``` -->
