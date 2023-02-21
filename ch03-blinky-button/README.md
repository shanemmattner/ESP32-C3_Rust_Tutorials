# Blinky Button Notes

We need to add a [pull-down resistor](https://www.electronics-tutorials.ws/logic/pull-up-resistor.html) to the button input pin.  Otherwise this pin will be [floating](https://www.mouser.com/blog/dont-leave-your-pins-floating) and our logic will not work right.

Add a big resistor like `10k ohms` connected to `ground`.  In the future I'll change the code to activate the internal pull-down resistor but right now I can't get it working.
