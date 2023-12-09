#!/bin/bash

curdir=$(pwd)
shldir=$(dirname $0)
cd $shldir
read -p "Your GitHub user name:" name
read -p "Your GitHub email address:" email
unset password
echo -n "Password for $email:"
while true; do
	read -n 1 -r -s char
	if [[ $char == $'\0' ]]; then
		break
	fi
	password+="$char"
	echo -n "*"
done
echo 
git config --global sendemail.confirm auto
git config --global sendemail.smtpserver smtp.office365.com
git config --global sendemail.smtpencryption tls
git config --global sendemail.smtpuser $email
git config --global sendemail.smtpserverport 587
git config --global sendemail.smtppass $password
git config --global user.email $email
git config --global user.name $name
git config --global user.signingkey $(head -n1 /root/.gnupg/signingkey.txt)
git config --global --add commit.gpgsign true
git remote set-url origin git@github.com:TaiseiIto/THEOS.git
cat .gitconfig >> /root/.gitconfig
cat ../.ssh/config >> /root/.ssh/config
chmod 600 /root/.github/key
chmod -R 600 /root/.gnupg
/root/.cargo/bin/cargo login $(cat /root/.crates.io/key)
cd /root/mikanos
git remote set-url origin git@github.com:TaiseiIto/mikanos.git
cd /root/osbook
git remote set-url origin git@github.com:TaiseiIto/mikanos-build.git
cd /root/edk2
git remote set-url origin git@github.com:TaiseiIto/edk2_for_mikanos.git
cd $curdir

