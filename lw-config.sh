#!/bin/bash

SELFPWD="$(realpath "$(dirname "$0")")"

# LANGUAGE=ru
TEXTDOMAIN=lw-config
TEXTDOMAINDIR="$SELFPWD/locale"

# SLINT=slint-viewer
SLINT=slint-viewer-qt
SLINTGUI="$SELFPWD/lw-config.slint"
GUICONFIG="$SELFPWD/lw-config.json"
CALLBACKSFL="/tmp/.slcl$BASHPID"

# unset QT_QPA_PLATFORMTHEME
# export SLINT_STYLE=fluent-dark
# export SLINT_STYLE=fluent-light
# export SLINT_STYLE=material
# export SLINT_STYLE=material-dark
# export SLINT_STYLE=material-light
# export SLINT_STYLE=cupertino
# export SLINT_STYLE=cupertino-dark
# export SLINT_STYLE=cupertino-light

SLINT_ARGS=(
   "$SLINTGUI"
   --translation-domain "$TEXTDOMAIN"
   --translation-dir "$TEXTDOMAINDIR"
   --load-data "$GUICONFIG"
   # --save-data "$GUICONFIG"
   --auto-reload
)

SLINT_CALLBACKS=(
   --on run_clicked    "sh -c 'echo 1 > $CALLBACKSFL'"
   --on reset_clicked  "sh -c 'echo 2 > $CALLBACKSFL'"
)

chngkv() { jq ".$1 = $2" "$GUICONFIG"|sponge "$GUICONFIG" ; }

lw-config() {
   "$SLINT" "${SLINT_ARGS[@]}" "${SLINT_CALLBACKS[@]}"
   sleep 0.1
   CALLBACKS="$(cat "$CALLBACKSFL" 2>/dev/null)"
   rm -f "$CALLBACKSFL"
   case "$CALLBACKS" in
      1) lwrap ;;
      2) echo '{}' > "$GUICONFIG"
         lw-config ;;
   esac
}

lw-config
#--------------------------------------------------------------------------
# jq ".wine_list = [$(jq '.wine_list[]' "$GUICONFIG"|tr '\n' ',')\"System\"]" "$GUICONFIG"
# jq ".wine_list = [$(jq '.wine_list[]' "$GUICONFIG"|tr '\n' ',')\"System\",\"System\"]" "$GUICONFIG"
