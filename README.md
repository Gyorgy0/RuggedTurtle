# RuggedTurtle

This is my first interpreter written in Rust. The program let's you control a turtle with specific commands. The program primarily is for teaching programming,
by visualizing instruction graphically.

#### WebAssembly version
<center>
  
  ```
  https://gyorgy0.github.io/RuggedTurtle-release/
  ```
  
</center>

# Programming documentation

The program supports multiple commands for controlling the turtle's movement. Valid commands are some hungarian and english keywords.

## Variables declaration
You can declare numbers as a variable or you can refer to a previously declared variable in a variable declaration.
-   ```<variable_name> = variable value```


## Arithmetic operations
- ```+``` - addition (it adds two numbers or variables together)

- ```-``` - subtraction (it subtracts a number from another number or variable)

- ```*``` - multiplication (it multiplies a number with another number or variable)

- ```/``` - full division (it divides a number with another number or variable, it doesn't neccesarily produce an integer)

- ```:``` - integer division (it divides a number with another number or variable, it produces an integer)

- ```%``` - remainder division (it divides a number with another number or variable, it gives back the remainder)


## Commands

- ```forward(number of pixels that needs to be travelled by the turtle)``` <br>**Aliases: e(), elore(), f(), forward()**

- ```right(angle in degrees, it specifies how much it needs to rotate to the character's right side 0-360)``` <br>**Aliases: j(), jobb(), jobbra(), r(), right()**

- ```left(angle in degrees, it specifies how much it needs to rotate to the character's left side 0-360)``` <br>**Aliases: b(), bal(), balra(), l(), left()**

- ```pencolor(red channel 0-255, green channel 0-255, blue channel 0-255, alpha channel 0-255) - specifies the color of the line e.g. (0,0,0,255) - black, (255,255,255,255) - white. (255,255,255,255) - transparent``` <br>**Aliases: tsz(), tollszin(), szin(), pc(), pencolor(), color()**

- ```penwidth(width of the pen in pixels, greater the value the thicker the line left behind)``` <br>**Aliases: tv(), tollvastagsag(), vastagsag(), pw(), penwidth(), width()**

- ```penup - the turtle lifts up it's pen from the canvas so it doesn't paints it's path on the canvas``` <br>**Aliases: tf, tollfel, pu, penup**

- ```pendown - the turtle puts down it's pen so it leaves it's path behind``` <br>**Aliases: tl, tollle, pd, pendown**

- ```evaluate(<variable>) - simplifies and prints out the value of the specified variable``` <br>**Aliases: kier(), kiertekeles(), kiszamolas(), eval(), calc(), calculate(), evaluate()**

- ```print(<variable>) - prints out the specified variable``` <br>**Aliases: ki(), kiir(), kiiratas(), print()**

- ```clear - clears the terminal history``` <br>**Aliases: trl, torol, clr, clear**

- ```reset - resets the application``` <br>**Aliases: alaphelyzet, reset, default**

- ```help - prints out the commands and their usage``` <br>**Aliases: ?, segitseg, help**

## Execution controls:

- ```repeat(variable, from, to (exclusive boundary - it goes until it hits the NUMBER BEFORE THE SPECIFIED number )) {commands}``` <br>**Aliases: i() {}, ism() {}, ismetles() {}, r() {}, rep() {}, repeat() {}, for() {}**

# Usage


https://github.com/user-attachments/assets/60537ea4-ade5-4772-9d83-4bc40853d0bc


# Graphics
Here are some commands and their effect on the turtle's movement.
```
for(i, 0, 200) {pencolor(55+i, 0, 55+i, 255); forward(30+i); right(59)}
```

<div style="text-align: center;">
  
![for(i, 0, 200) {pencolor(55+i, 0, 55+i, 255); forward(30+i); right(59)} command output](https://github.com/user-attachments/assets/7279bcd8-bc5a-4d4e-a53d-59a39947224a)

</div>


```
for(i, 0, 200) {forward(100); right(59+i); forward(100)}
```

<div style="text-align: center;">

![for(i, 0, 200) {forward(100); right(59+i); forward(100)} command output](https://github.com/user-attachments/assets/9e564e8c-c70e-477a-b790-710d905ade7a)

</div>

# Arithmetics

<div style="text-align: center;">
  
  ![Arithmetics output](https://github.com/user-attachments/assets/493d97c1-ab1b-4711-a930-576463085e9a)


</div>
