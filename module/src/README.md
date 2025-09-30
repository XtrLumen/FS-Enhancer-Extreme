#Tricky Store Enhancer Extreme模块

## 简介
提升**TrickyStore**的使用体验,同时**极致隐藏**由**解锁引导加载程序**产生的**相关检测点**.

## 功能详情
注释#做了调用没WEBUI
注释^纯WEBUI功能没做
## 计划由WebUI调用的已完成后端功能
联网更新安全补丁时间: [ tseed --securitypatchdatefetch ]
抓取有效密钥: [ tseed --stealkeybox < Basic |-a|-b|-c| > ]
TrickyStore服务状态读取/控制: [ tseed --tsctl < Basic |-stop|-start|-state| > ]
TSEnhancerExtreme服务状态读取/控制: [ tseed --tseectl < Basic |-stop|-start|-state| > ]
## 完全由WebUI实现的未完成前端功能
快捷额外添加系统应用
快捷额外移除第三方应用
快捷自定义安全补丁时间
快捷从设备存储导入Keybox
## 已完成功能
检测冲突模块并添加移除标签/直接移除,于安装/开机/安装新模块时执行检测
自动备份,卸载时恢复备份,并在旧备份无Keybox时完成最后一次替换
AVB2.0被关闭等VerifiedBootHash状态异常时全自动修正
根据区域划分为中文/英文提示: 运行状态/安装过程
TS本体卡片位置Action按钮,用于更新包名
Action按钮用于Magisk启动WEBUI
设定Bootloader的属性值为锁定
将安全补丁时间同步到属性值
Dex后台服务,闲时零功耗
谷歌根证书签名Keybox
安装过程更新包名
每次启动更新包名
显示模块运行状态

## 使用方式
由于**WEBUi没有完成制作**,暂时只能**手动修改配置文件**.
- **配置目录**路径: [ /data/adb/ts_enhancer_extreme/ ].
- **sys.txt**中填入想**额外加入**的**系统应用**的**包名**
- **usr.txt**中填入想**额外去除**的**用户应用**的**包名**
- **Tsee cli**使用方式: 执行[ PATH="/data/adb/modules/ts_enhancer_extreme/binaries:$PATH" ]后: [ tseed -h ]自行**查看参数支持**.
## 提示
- **log**文件夹中有**日志记录**,如**遇到问题**可以**自行排查**,如**解决不了**请进入**TG@cirnoclass**反馈.
- **安装时自动生成**的**备份配置目录**路径: [ /data/adb/tricky_store/config_backup/ ],是**安装本模块前**的**配置目录**.