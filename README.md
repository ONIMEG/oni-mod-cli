用于创建 MSBuild 类型的缺氧模组项目  
使用了 [@peterhaneve 缺氧模组代码仓库](https://github.com/peterhaneve/ONIMods)的模板，有些地方做了一些改动
## 使用
### 创建解决方案
先在文件资源管理器打开想要创建解决方案的位置，然后在选定位置打开命令行，输入以下命令：
```cmd
oni-mod-cli create sln <name>
```
其中 `name` 字段可选，可根据命令行提示完成创建流程

### 创建项目
在包含 `*.sln` 文件的目录下打开命令行，输入以下命令：
```cmd
oni-mod-cli create csproj <name>
```
`name` 字段同样可选
