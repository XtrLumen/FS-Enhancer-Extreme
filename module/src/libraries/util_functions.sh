##VARIABLE##
#ALIAS#
TS="tricky_store"
TSEE="ts_enhancer_extreme"
S="service.sh"
D=".tsee_state.sh"
P="post-fs-data.sh"
#ZERO LEVEL#
ADB="/data/adb"
#ONE LEVEL#
MODULESDIR="$ADB/modules"
TSEECONFIG="$ADB/$TSEE"
SD="$ADB/service.d"
#TWO LEVEL#
TSEEMODDIR="$MODULESDIR/$TSEE"
TSMODDIR="$MODULESDIR/$TS"
#THREE LEVEL#
MULTIPLETYPE="$TSEECONFIG/multiple.txt"
KERNELTYPE="$TSEECONFIG/kernel.txt"
TSEELOG="$TSEECONFIG/log/log.log"
TSEEBIN="$TSEEMODDIR/binaries"
TYPE="$TSEECONFIG/root.txt"
#OTHERS#
ORIGIN=$(basename "$0")
##END##

##FUNCTIONS##
#MULTILINGUAL#
[[ "$(getprop persist.sys.locale)" == *"zh"* || "$(getprop ro.product.locale)" == *"zh"* ]] && LOCALE="CN" || LOCALE="EN"
operate() {
  if [ "$LOCALE" = "$1" ]; then
    shift
    local operation="$1"
    shift
    case "$operation" in
      echo)
        if [ "$1" = "-n" ]; then
          shift
          echo -n "$@"
        else
          echo "$@"
        fi
        ;;
      functions)
        eval "${1%=*}=\"${1#*=}\""
        ;;
    esac
  fi
}
echo_cn() { operate "CN" "echo" "$@"; }
echo_en() { operate "EN" "echo" "$@"; }
#OTHER#
logout() { echo "$(date "+%m-%d %H:%M:%S.$(date +%3N)")  $$  $$ I System.out: [TSEE]$1" >> "$TSEELOG"; }
logc() { logout "<CLI>$1"; }
logs() { logout "<Service>$1"; }
logd() { logout "<Service.D>$1"; }
logp() { logout "<Post-Fs-Data>$1"; }
invoke() {
  case "$ORIGIN" in
    *"$S"*)
      class="logs"
      ;;
    *"$P"*)
      class="logp"
      ;;
    *"$D"*)
      class="logd"
      ;;
  esac
  "$class" "$1"
  if $TSEEBIN/tseed $2; then
    "$class" "完毕"
  else
    "$class" "失败"
  fi
}
check() {
  if [ "$(cat "$TYPE")" = "Multiple" ] || [ ! -d "$TSMODDIR" ] || [ -f "$TSMODDIR/disable" ]; then
    case "$ORIGIN" in
      *"$P"*)
        logp "环境异常,拦截执行"
        rm -f "$TSMODDIR/action.sh"
        mv "$TSEEMODDIR/webroot" "$TSEEMODDIR/.webroot"
        mv "$TSEEMODDIR/action.sh" "$TSEEMODDIR/.action.sh"
        ;;
      *"$S"*)
        exit
        ;;
    esac
  else
    if [[ "$ORIGIN" == *"$P"* ]]; then
      logp "环境正常,继续执行"
      mv "$TSEEMODDIR/.webroot" "$TSEEMODDIR/webroot"
      mv "$TSEEMODDIR/.action.sh" "$TSEEMODDIR/action.sh"
      ln -sf "$TSEEMODDIR/libraries/action.sh" "$TSMODDIR/action.sh"
    fi
  fi
}
detect() {
  if [ $? -eq 0 ]; then
    echo_cn "完毕"
    echo_en "Complete"
  else
    echo_cn "失败"
    echo_en "Failed"
  fi
}
initwait() { resetprop -w sys.boot_completed 0; }
##END##

if [[ "$ORIGIN" == *"$P"* ]]; then
  rm -f "$MULTIPLETYPE"
  rm -f "$KERNELTYPE"
  rm -f "$TSEELOG"
  rm -f "$TYPE"
else
  ROOT=$(cat "$TYPE")
fi