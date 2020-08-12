tergent - a cryptoki library for termux keystore
------------------------------------------------

A [cryptoki/PKCS#11 library](http://docs.oasis-open.org/pkcs11/pkcs11-base/v2.40/os/pkcs11-base-v2.40-os.html) for [Termux](https://termux.com/) that uses [Android Keystore](https://developer.android.com/training/articles/keystore) as its backend.

This library enables the use of keys securely stored in termux-api with PKCS#11 protocol capable applications. These include the applications provided by openssh, such as `ssh` and `scp`.

Tergent does not (and cannot) access your private keys as they are stored inside the secure hardware. In fact, they can never leave the chip even with root privileges, thanks to [extraction preventation](https://developer.android.com/training/articles/keystore#ExtractionPrevention).  
Cryptographic actions are performed by the hardware itself.

This library is aimed to be compliant with PKCS#11 version 2.40. For now it implements all the APIs required for `ssh` and the related applications to function. If you encounter any issues trying to use tergent with any other PKCS#11 capable application, please open a bug report.

Compiling
---------
Install [Rust](https://www.rust-lang.org/en-US/install.html) and [Android NDK](https://developer.android.com/ndk/).  
You will need to configure cargo with the correct locations for "ar" and "linker", you can follow this page up to and including the `rustup target add ...` command:  
[https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html](https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html)  
Then this project can be compiled with the command `cargo build --target=aarch64-linux-android` (or any other Android target).

Alternatively, you can download a precompiled deb package from the releases page.

Upgrading from 0.1
------------------
Keys generated for tergent 0.1 will not work for the latest version. If you do not want to lose access to your server, generate new keys using step 2 below and copy it to your server before upgrading.  
If you have already upgraded, you can download a copy of an older version from the releases page.  
After upgrading, any modifications made in `.bash_profile` or similar files should be removed. There is no need to keep running a background process anymore.

Usage
-----
1. Make sure you have the latest version of [Termux:API](https://play.google.com/store/apps/details?id=com.termux.api) installed. Don't forget to install the scripts using the command `pkg install termux-api`.

2. As of now `termux-keystore` cannot generate keys compatible with tergent. Use one of these commands instead:
  - To generate an RSA key:
```
/data/data/com.termux/files/usr/libexec/termux-api Keystore -e command generate -e alias ALIAS -e algorithm ALGORITHM --ei purposes 12 --esa digests NONE,SHA-1,SHA-256,SHA-384,SHA-512 --ei size SIZE --ei validity VALIDITY
```
  - To generate an EC key:
```
/data/data/com.termux/files/usr/libexec/termux-api Keystore -e command generate -e alias ALIAS -e algorithm EC --ei purposes 12 --esa digests NONE,SHA-1,SHA-256,SHA-384,SHA-512 -e curve CURVE --ei validity VALIDITY
```
  - ALIAS is the name you want to give to the key.
  - SIZE can be 2048, 3072 or 4096, this is only used for RSA keys.
  - CURVE can be secp256r1, secp384r1 or secp521r1, this determines the EC key size.
  - VALIDITY can be used for user validity, see Auto-locking below. Use 0 to disable.

3. List the keys to verify using the standard ssh tool: `ssh-keygen -D $PREFIX/lib/libtergent.so`. Even though the command is called `ssh-keygen`, `-D` switch lists cryptoki keys instead of generating new keys.

4. Copy the public key to your server.
    1. First export the public keys using: `ssh-keygen -D $PREFIX/lib/libtergent.so > keys.pub`.
    2. Optionally, remove any keys from the generated file using a text editor if needed: `nano keys.pub`.
    3. Copy the keys to your server using `ssh-copy-id -f -i keys.pub example.com`.

5. Connect to your server using the command `ssh -I $PREFIX/lib/libtergent.so example.com`.

To make `ssh` remember the library path, modify your `~/.ssh/config` file. For a single host:
```
Host example.com
	PKCS11Provider /data/data/com.termux/files/usr/lib/libtergent.so
```
Or alternatively you can make tergent apply to all connections:
```
Host *
	PKCS11Provider /data/data/com.termux/files/usr/lib/libtergent.so
```
After saving this file you can just run `ssh example.com` to connect.

How do I...
-----------
* **list keys**: run either `ssh-keygen -D $PREFIX/lib/libtergent.so` or `termux-keystore list`
* **create a new key**: see step 2 from instructions above
* **use a key**: run `ssh -I $PREFIX/lib/libtergent.so`
* **delete a key**: use `termux-keystore delete`
* **import a key**: not supported, generate a new key instead

Auto-locking
------------
tergent does not provide password protected sessions yet.
However, Android [provides a mechanism](https://developer.android.com/training/articles/keystore#UserAuthentication) to automatically lock the keys after a specified time has passed since the last device unlock. To take advantage of this feature, use the flag while generating the keys, e.g. `--ei validity 10` for a 10-second lock. In this case, the keys are usable only for 10 seconds after the phone is unlocked. To unlock the keys after this time has passed, simply re-lock and unlock your device again.
