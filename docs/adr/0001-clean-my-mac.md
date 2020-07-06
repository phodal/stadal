# 1. clean my mac

日期: 2020-07-06

## 状态

2020-07-06 提议

## 背景

Refs: https://github.com/Kevin-De-Koninck/Clean-Me/blob/master/Clean%20Me/Paths.swift

dir:

```

let globalTempFilesPath     = "/tmp/"
let userCachePath           = "~/Library/Caches/"
let userLogsPath            = "~/Library/logs/"
let userPreferencesPath     = "~/Library/Preferences/"
let systemLogs1Path         = "/Library/logs/"
let systemLogs2Path         = "/var/log/"
let mailAttachementsPath    = "~/Library/Containers/com.apple.mail/Data/Library/Mail\\ Downloads/"
let trashPath               = "~/.Trash/"
let xcodeDerivedDataPath    = "~/Library/Developer/Xcode/DerivedData/"
let xcodeArchivesPath       = "~/Library/Developer/Xcode/Archives/"
let xcodeDeviceLogsPath     = "~/Library/Developer/Xcode/iOS\\ Device\\ Logs/"
let terminalCacheFilesPath  = "/private/var/log/asl/*.asl"
let terminalCachePath       = "/private/var/log/asl/" // used for open func
let bashHistoryFile         = "~/.bash_history"
let bashHistoryPath         = "~/.bash_sessions/"
let downloadsPath           = "~/Downloads/"
let userAppLogsPath         = "~/Library/Containers/*/Data/Library/Logs/"
let userAppCachePath        = "~/Library/Containers/*/Data/Library/Caches/"
let spotlightPath           = "/.Spotlight-V100/"
let docRevPath              = "/.DocumentRevisions-V100/"
let imessageAttachmentsPath = "~/Library/Messages/Attachments/"
```

with Command:

```
sudo du -sh /var
```

Get All Homebrew

https://stackoverflow.com/questions/40065188/get-size-of-each-installed-formula-in-homebrew

```
brew list | xargs brew info | egrep --color '\d*\.\d*(KB|MB|GB)'
```

## 决策

在这里补充上决策信息...

## 后果

在这里记录结果...
