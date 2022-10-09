# SplatGear roll helper

A small Rust program to find the best solution for desired ability within next 5 rolls

## How to use

1. Calculate the current seed of your gear, in this link: <https://leanny.github.io/splat3seedchecker/#/seedrecover>. You may need many rolls to get the correct seed, but it's worth it.
2. Run the program, input seed, brand, your target ability, and the base quality. Quality means the order of target ability, A stands for target, B stands for other ability. So AAA is best, AA is good, ABA is just common.
3. The program will output the best solution within 5 rolls for you. The cost means how many drink tickets you need to get the solution. And the sequence means what drink should in effect when the roll happens. 'Appear at' means the ability ordering will start at where in the sequence.

## Let's take an example

With seed 0x41215, brand 2, ability 5, quality AA, we will get this at the top of the output:

```shell
Quality: AA,    Cost: 1,        Drink:"None, None, None, None, SpecialCharge",  Appear at: 3
```

This means if keep no drinking for the next 4 rolls, and drink SPChargeUp during the 5th roll, you will get two SPChargeUp at the last two rolls. Notice that the sequence number starts from 0, so appear at 3 means AA starts from the 4th roll.
So you should clear all the abilities before the 4th roll, **insert SPChargeUp with 10 frags in the first slot if you want to get AAA** and drink SPChargeUp during the 5th roll.

Similarly, if the best you get is ABA, you should clear the gear before 1st A, and swap the B with 20 frags in the 2nd slot.
