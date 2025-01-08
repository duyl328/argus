# WPF 问题及解决方案

### Wpf 的 MVVM 中 RelativeSource 属性绑定及跨级无法绑定的问题

**介绍**
RelativeSource 是一个标记扩展，扩展描述相对于绑定目标位置的绑定源位置。

RelativeSource 作用是改变绑定指向的源，默认使用绑定的源是对应的DataContext对象中的属性，如果想要绑定源为其他元素，比如自身、其他父级控件、其他ViewModel，就需要用RelativeSource 进行标记。它表示在绑定表达式中引用另一个元素的属性。它使开发人员可以根据元素的位置和类型来查找绑定源。

**RelativeSource属性介绍**

|      属性       |                             解释                             |
| :-------------: | :----------------------------------------------------------: |
|  AncestorLevel  | 以 FindAncestor 模式获取或设置要查找的上级级别。 使用 1 指示最靠近绑定目标元素的项。 |
|  AncestorType   |                  指定查找的上级节点的类型。                  |
|      Mode       | 获取或设置 RelativeSourceMode 值，该值描述相对于绑定目标的位置的绑定源的位置。Mode属性是一个RelativeSourceMode 枚举 |
|  PreviousData   | 一个静态值，效果等同于 Mode=PreviousData。该值用于返回为 RelativeSource 模式构造的 PreviousData。 |
|      Self       | 一个静态值，效果等同于 Mode=Self。该值用于返回为 RelativeSource 模式构造的 Self。 |
| TemplatedParent | 一个静态值，效果等同于 Mode=TemplatedParent。该值用于返回为 RelativeSource 模式构造的 TemplatedParent。 |

`RelativeSource` 的 `Mode` 属性有四种模式：

`FindAncestor`：查找指定类型的父元素，和 AncestorType 或 AncestorLevel 一起使用。
`PreviousData`：使用数据提供程序中的前一个数据项。介绍
`Self`：使用当前元素。
`TemplatedParent`：查找元素的模板父级。

**四种模式用法:**
**Self模式**
绑定控件本身的属性

示例：

实现正方形控件（宽和高相等），通过RelativeSource 实现，使用Self将Width绑定到Height上。

```c#
Width="{Binding RelativeSource={RelativeSource Self}, Path=Height}"
```

或者
```c#
Width="{Binding RelativeSource={RelativeSource Mode=Self}, Path=Height}
```



控件的Xaml代码：

```xml
 <Button
            Width="{Binding RelativeSource={RelativeSource Mode=Self}, Path=Height}"
            Height="100"
            Margin="40,67,0,0"
            HorizontalAlignment="Left"
            VerticalAlignment="Top"
            Content="Button"/>
```



**FindAncestor模式**
绑定控件父元素及更上一级的元素，使用Mode=FindAncestor和AncestorType 、 AncestorLevel配合实现。

**使用规则：**
必须要设置AncestorType 属性，只要设置了Mode=FindAncestor 和 AncestorLevel值，就必须要设置AncestorType 属性，否则会报错。
如果未显式设置属性 Mode，则设置AncestorType 或 AncestorLevel属性会将属性值隐式锁定 Mode 为 FindAncestor。（Mode=FindAncestor可省略）
AncestorLevel设置为1就是默认找到的第一个指定类型的父元素，2就是第二个
绑定成功的还有一点，对应父元素相关属性必须有值，如果对应父元素对应属性无值，则无效果。
如下几种定义效果是相同的

```xml
RelativeSource={RelativeSource Mode=FindAncestor, AncestorLevel=1, AncestorType=Grid}
RelativeSource={RelativeSource AncestorLevel=1, AncestorType=Grid}
RelativeSource={RelativeSource AncestorType=Grid}
```

**PreviousData模式**

PreviousData可以在数据绑定表达式中使用前一个数据项。通常在ItemsControl中使用，用于根据前一个项的属性计算当前项的属性。PreviousData**只能**在ItemsControl或其派生类中使用。



**特殊注意事项：**

#### MVVM嵌套绑定事件不生效.

使用RelativeSource改变绑定的ViewModel对象。在嵌套的ViewModel中，命令一定要写在对应的VIewModel（或者Model）中才能实现绑定，这时可以通过RelativeSource绑定到其父级的ViewModel中的命令上。

**要绑定到其他ViewModel中的属性或者命令，主要在Binding中的Path里面加上`DataContext.`，就能绑定，如`Path=DataContext.XXX`**

比如在DataGrid列中有按钮事件，就必须对应的命令需要写在对应的Model中增加命令

正常下一个DataGird（在用户控件UserManageUC.xaml中）

```xml
<DataGrid
            Width="605"
            Height="341"
            Margin="59,74,0,0"
            HorizontalAlignment="Left"
            VerticalAlignment="Top"
            AutoGenerateColumns="False"
            ItemsSource="{Binding Users}">
            <DataGrid.Columns>
                <DataGridTextColumn Binding="{Binding Id}" Header="序号" />
                <DataGridTextColumn Binding="{Binding Name}" Header="姓名" />
                <DataGridTextColumn Binding="{Binding Age}" Header="年龄" />
                <DataGridTemplateColumn Header="操作">
                    <DataGridTemplateColumn.CellTemplate>
                        <DataTemplate>
                            <StackPanel Orientation="Horizontal">
                                <Button Command="{Binding ModifyCmd}" Content="修改" />
                                <Button Content="删除" />
                            </StackPanel>
                        </DataTemplate>
                    </DataGridTemplateColumn.CellTemplate>
                </DataGridTemplateColumn>
            </DataGrid.Columns>
        </DataGrid>
```

**以上这段代码在实际过程中是无效的，绑定的事件无法找到对应的VM，造成事件无法绑定**



**解决方式：**

> 改变按钮Command的绑定，使用 `RelativeSource` 的 `FindAncestor` 模式
>
> RelativeSource={RelativeSource AncestorType=local:UserManageUC}
>
> 帮绑定对象改为 UserManageUC 用户控件，然后目标改为 DataContext.ModifyCmd，然后增加CommandParameter，将该行绑定的数据作为命令参数传入

```xml
  <Button
        Command="{Binding DataContext.ModifyCmd, RelativeSource={RelativeSource AncestorType=local:UserManageUC}}"
        CommandParameter="{Binding}"
        Content="修改" />
```

这里的注意点是一定要在绑定的内容前加入 **`DataContext`** 进行指定，否则绑定无效！