#!/bin/sh

# Install package dependencies.
apt -y update && apt -y install curl gcc p7zip-full

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Wrap SteamCMD binary (use Wine)
cat << EOF > /usr/bin/steamcmd
#!/bin/sh
sudo -u games wine /usr/games/Steam/steamcmd.exe "\$@"
EOF

chmod 755 /usr/bin/steamcmd

# Suppress ALSA lib warnings.
mkdir /usr/games/.config/pulse && chown games:games /usr/games/.config/pulse
