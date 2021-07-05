### 树莓派时钟


通过树莓派4b和max7219 实现

![clock](clock.gif)

运行需要指定 Max7219 `data`、`cs`、`clk`对应树莓派引脚编号，如

```shell
./raspi_clock 24 23 18
```