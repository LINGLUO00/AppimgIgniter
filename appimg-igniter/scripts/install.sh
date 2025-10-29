#！ /bin/bash
set -Eeuxo pipefail #any step fails, the script will exit

PREFIX="/usr/local"

while [$# -gt 0]; do #$@ is all the arguments passed to the script
    case "$1" in
        --prefix)
            PREFIX="$2"
            shift 2;;
        --prefix=*)
            PREFIX="${1#*=}"
            shift;;
        *)
            shift;;
    esac
done

# 3. 计算若干目标路径
BIN="$PREFIX/bin/appimg-igniter"               # 可执行文件
BINFMT="/etc/binfmt.d/appimgigniter.conf"      # binfmt 规则文件
MIME_DST="/usr/share/mime/packages/appimage.xml"
DESKTOP_DST="/usr/share/applications/appimg-igniter.desktop"

echo "==> 安装可执行文件到 $BIN"
install -Dm755 target/release/appimg-igniter "$BIN"

echo "==> 写入 binfmt 配置到 $BINFMT"
cat >"$BINFMT" <<EOF
:appimage-type1:M:8:AI\\x01::$BIN:F
:appimage-type2:M:8:AI\\x02::$BIN:F
EOF
systemctl restart systemd-binfmt.service

echo "==> 安装 MIME 类型描述"
install -Dm644 resources/mime/packages/appimage.xml "$MIME_DST"
update-mime-database /usr/share/mime

echo "==> 安装 .desktop 启动器"
install -Dm644 /dev/stdin "$DESKTOP_DST" <<EOF
[Desktop Entry]
Name=AppImage Igniter
Exec=$BIN %f
Icon=appimage
Type=Application
NoDisplay=true
MimeType=application/vnd.appimage;
EOF

update-desktop-database /usr/share/applications

