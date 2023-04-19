---
title: "Compile zfs on mainline kernel"
date: 2022-03-17T16:11:15+02:00
draft: true
---

Recently I got a newly released GPU. To use it on Linux however I needed the latest kernel and mesa driver. The latter was straightforward however the kernel proved tricky as I use ZFS. While there are a number of text how to compile a kernel a guide on how to do that _and fold in ZFS_ was missing. This guide takes you through the process of building both ZFS and a mainline kernel from source from my perspective, that of an intermediate Linux user.

### Sources and dependencies
Check te META file from [OpenZFS on Linux](https://github.com/openzfs/zfs) for the maximum supported Linux version. 

get the requirements to build [OpenZFS](https://openzfs.github.io/openzfs-docs/Developer%20Resources/Building%20ZFS.html#installing-dependencies):
`sudo apt install build-essential autoconf automake libtool gawk alien fakeroot dkms libblkid-dev uuid-dev libudev-dev libssl-dev zlib1g-dev libaio-dev libattr1-dev libelf-dev linux-headers-generic python3 python3-dev python3-setuptools python3-cffi libffi-dev python3-packaging git libcurl4-openssl-dev`

get the requirements to build the kernel (these overlap with the reqs for OpenZFS):
`sudo apt-get install git fakeroot build-essential ncurses-dev xz-utils libssl-dev bc flex libelf-dev bison`

Get a kernel tar.xz from [kernel.org](kernel.org) with a version equal to or lower then the maximum supported by _OpenZFS on Linux_.
I recommend storing it in a new build dir in the root of the file system (thus `/build`). Assuming you do not use zfs on root you can still retry the build if zfs no longer works.
unzip with `tar -xf linux-{version}.tar.xz`


### Building the Kernel
First we configure the kernel.
copy the current kernels config
`cp /boot/config-$(uname -r) .config`

Optional: configure the kernel and remove functionality you do not need using:
```bash
make menuconfig
```
Do not forget to save before closing menuconfig.

Compile the kernel 
run: 
```bash
make -j $(nproc)
```

The argument `-j $(nproc)` tells _make_ to use all hardware threads. This speeds things up quite a bit on a modern machine. Still building a kernel takes a while, go grab a book! 

Unfortunately there is not a progress bar. Compilation is getting to the end once lines no longer contain the word driver. Once most lines start with LD (linking) instead of CC (compiling) it will be done any second.

Once the kernel has been build we make the normal (in tree) kernel modules. We will add ZFS to those in the next section (except zfs):
```
cd to kernel build folder
sudo make modules_install -j $(nproc) 
```

#### Certificate error
Depending on the setup of your machine you can run into a certificate error which looks like this:
```
make[2]: *** No rule to make target 'debian/canonical-certs.pem', needed by 'certs/x509_certificate_list'.  Stop.
```
and/or (possibly only after fixing the one above)
```
make[2]: *** No rule to make target 'debian/canonical-revoked-certs.pem', needed by 'certs/x509_revocation_list'.  Stop.
```
This certificate is used to verify the kernels source has not been tempered with. It was used while building your currently running kernel. By copying the current kernel's config we got the need for this certificate. It is however not present on our machine. The proper way to solve this would be to find a trustworthy source for a certificate signing the kernel source. The simplest approach is to disable signature checking. We choose to trust our source of the source code ([kernel.org](kernel.org)) and disable signature checking:

run 
```
make menuconfig
```
In _menuconfig_ navigate to:
`Cryptographic API` -> `Certificates for signature checking`
Now find the line ending in `Additional X.509 keys for default system keyring` and set its value to nothing. You will probably need to do the same for the line ending in `X.509 certificates to be preloaded into the system blacklist`. Now _save_ and _exit_ _menuconfig_ then continue building the kernel.

### Building OpenZFS
now we can build OpenZFS, here we mostly follow the instructions on [openzfs.github.io](https://openzfs.github.io/openzfs-docs/Developer%20Resources/Building%20ZFS.html#configure-and-build)
execute `sh autogen.sh`
then call ./configure --with-linux=/build/linux-{version} --with-linux-obj=/build/linux-{version}
the build the kernel with make -s -j$(nproc)

now we build the ZFS kernel module
```
cd to zfs build folder
sudo make install
```

#### Verify the versions
check that the zfs module exists: /lib/modules/<kernel version>/extra/zfs.ko
check its kernel version matches: `modinfo /lib/modules/<kernel version>/extra/zfs.ko`
this should output something containing: vermagic: <kernel version>

### Combining kernel and ZFS
Here we combine the kernel and its modules (now containing ZFS) to an initial ram file system: `initrd.img`. Luckily for us, the kernels make file can do this for us. The kernel modules however contain a ton of debug information in there present state. If we were to build an `initrd.img` now it would be around 800 Megabytes (MB). The typical boot partition is only around 300 MB.

To strip the debug info from all the kernel modules we run `strip`.
```
cd /lib/modules/<new_kernel>
sudo find . -name "*.ko" -exec strip --strip-unneeded {} +
```
Now the resulting `initrd.img` will be around 120 MB.

Lets build and install it! Move back to the root of the kernel sources you downloaded (for me that is `/build/linux-<version>`). Then run:
```
sudo make install -j $(nproc)
```

Take care to ensure `initrd.img-previous` and `vmlinuz-previous.efi` are set to a working kernel, if you are trying this for a second time (see Troubleshooting EFI full).

Reboot!
Remember if anything went wrong you should be able to boot to your previous kernel. 

## Troubleshooting
### the zfs pool is not imported on boot
`zfs-import-cache` systemd service not running with error: ConditionFileNotEmpty=/usr/local/etc/zfs/zpool.cache. That service runs on boot and imports the pool by running zpool import. It looks like this:

```
[Unit]
Description=Import ZFS pools by cache file
Before=zfs-import.target
ConditionFileNotEmpty=/usr/local/etc/zfs/zpool.cache
<and more>

[Service]
ExecStart=/usr/local/sbin/zpool import -c /usr/local/etc/zfs/zpool.cache -aN $ZPOOL_IMPORT_OPTS
<and more>
```

From the systemd [docs](https://www.freedesktop.org/software/systemd/man/systemd.unit.html) we learn ConditionFileNotEmpty makes the service run only if the `zpool.cache` file exists and is non empty. 

If we get the 

### EFI full, too large initrd.img
sort all the files in the efi partition by size:
```bash
du -h /boot/efi | sort -n | tail -10
```
in my case the initrd-previous.img file was over 700MB, even for my huge (normal efi partitions are 100MB) efi partition this is too large. It was that large because I forgot the strip the debug symbols on a previous kernel.

You do not want to boot a kernel you just compiled without a previous working kernel to fall back on. Luckily quite a few initrd images are kept in /boot. Just copy over a known working image thats not huge, and dont forget the vmlinuz file!

For example:
```
sudo cp /boot/initrd.img-6.1.6-76060106-generic /boot/efi/EFI/Pop_OS-63ad2a51-0406-4edc-9ba5-59756de1bf48/initrd.img-previous
sudo cp /boot/vmlinuz-6.1.6-76060106-generic /boot/efi/EFI/Pop_OS-63ad2a51-0406-4edc-9ba5-59756de1bf48/vmlinuz-previous.efi
```
