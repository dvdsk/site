---
title: "Compile zfs on mainline kernel"
date: 2022-03-17T16:11:15+02:00
draft: true
---

Check te META file from [OpenZFS on Linux](https://github.com/openzfs/zfs) for the maximum supported Linux version. 

get the requirements to build [OpenZFS](https://openzfs.github.io/openzfs-docs/Developer%20Resources/Building%20ZFS.html#installing-dependencies):
`sudo apt install build-essential autoconf automake libtool gawk alien fakeroot dkms libblkid-dev uuid-dev libudev-dev libssl-dev zlib1g-dev libaio-dev libattr1-dev libelf-dev linux-headers-generic python3 python3-dev python3-setuptools python3-cffi libffi-dev python3-packaging git libcurl4-openssl-dev`

get the requirements to build the kernel (these overlap with the reqs for OpenZFS):
`sudo apt-get install git fakeroot build-essential ncurses-dev xz-utils libssl-dev bc flex libelf-dev bison`

get the kernel tar.xz from kernel.org, 
I recommend storing it in a new build dir in the root of the file system (thus `/build`). Assuming you do not use zfs on root you can still retry the build if zfs no longer works.
unzip with `tar -xf linux-{version}.tar.xz`

First we configure the kernel.
copy the current kernels config
`cp /boot/config-$(uname -r) .config`

you could now configure the kernel and remove everything you dont need using
make menuconfig

compile using `make -j $(nproc)` 
the `-j $(nproc)` ensures we use all hardware threads speeding things up quite a bit on a modern machine even then go grab a book this is going to take a while, about an hour on my machine. Unfortunately there isnt really a progress bar. You know it is almost done once most lines start with LD (linking) instead of CC (compiling). Most time will be spend compiling drivers, once thats done you are almost there!

you might run into an certificate error which looks like this:
`make[2]: *** No rule to make target 'debian/canonical-certs.pem', needed by 'certs/x509_certificate_list'.  Stop.`
and or (possibly later)
`make[2]: *** No rule to make target 'debian/canonical-revoked-certs.pem', needed by 'certs/x509_revocation_list'.  Stop.`
The proper way to solve this would be to find a list of certificates and revoked certificates from a trusted source (probably the current kernel sources/headers from your distro) the simplest approach is to disable signature checking:

In which case do make menuconfig go to:
Cryptographic API -> Certificates for signature checking
then find the line ending in `Additional X.509 keys for default system keyring` and set the sting to nothing`
if you also get the second error do the same for the line ending in `X.509 certificates to be preloaded into the system blacklist`
_do not forget to save before exiting_


now we can build OpenZFS, here we mostly follow the instructions on [openzfs.github.io](https://openzfs.github.io/openzfs-docs/Developer%20Resources/Building%20ZFS.html#configure-and-build)
execute `sh autogen.sh`
then call ./configure --with-linux=/build/linux-{version} --with-linux-obj=/build/linux-{version}
the build the kernel with make -s -j$(nproc)

now we build the zfs kernel module
```
cd to zfs build folder
sudo make install
```

now we make the normal (in tree) kernel modules (except zfs):
```
cd to kernel build folder
sudo make modules_install -j $(nproc) 
```

we strip the modules of their debug info making them around 10x smaller (otherwise the final kernel+modules would be over 500 MB!):
```
cd /lib/modules/<new_kernel>
sudo find . -name "*.ko" -exec strip --strip-unneeded {} +
```

now we can install the kernel with all the modules. That means build the initial ram file system from the compiled kernel and its modules:
```
sudo make install -j $(nproc)
```


## Verify everything should work
check that the zfs module exists: /lib/modules/<kernel version>/extra/zfs.ko
check its kernel version matches: `modinfo /lib/modules/<kernel version>/extra/zfs.ko`
this should output something containing: vermagic: <kernel version>

## Debugging
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

### Efi full, too large initrd.img
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
