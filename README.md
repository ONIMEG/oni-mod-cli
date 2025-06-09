用于创建 MSBuild 类型的缺氧模组项目  
使用了 [@peterhaneve 缺氧模组代码仓库](https://github.com/peterhaneve/ONIMods)的模板，有些地方做了一些改动
**那么使用所谓的 MSBuild 类型的缺氧模组项目有什么好处？**  
那就是能够自定义构建流程 
目前 @peterhaneve 的这一套构建流程能够实现：
1. 将 PLib 与你写的代码编译成一个 dll 文件
2. 能够自动生成模组配置文件
3. debug 模式下自动将模组文件移动到缺氧的模组 dev 文件夹中

## 安装
运行 oni-mod-cli-setup.exe 安装之后，需要[将安装目录添加到 Path 环境变量中](https://blog.csdn.net/chenhao0568/article/details/133167667)
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

## 配置文件的维护
首先 `Directory.Build.props.default` 文件中定义了游戏的安装目录以及模组文件夹，如果你在安装过程中修改了这些目录，自己创建一个 `Directory.Build.props.user`  文件，修改 `Directory.Build.props.default` 中的值  
`Directory.Build.props` 文件中定义的是整个解决方案的引用以及其他常量，其中
```xml
<PropertyGroup>
    <March25Qof>659901</March25Qof>
    <LatestBuild>$(March25Qof)</LatestBuild>
</PropertyGroup>
```
对应的是 mod_info.yml 中 minimumSupportedBuild 字段的值，如果你查看过由该程序创建的 csproj 就会发现 csproj 中有 `<LastWorkingBuild>$(LatestBuild)</LastWorkingBuild>` 这一字段，在上面的代码片段中 `<March25Qof>659901</March25Qof>` 中的值传递到了 `LatestBuild`，最终传递到 `LastWorkingBuild`。方便控制模组配置文件的 minimumSupportedBuild 字段。
