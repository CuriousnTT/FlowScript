# FlowScript ***#WIP***
The programming language designed for reactive *flow*!
FlowScript will be built from scratch to resemble TypeScript, and aims to require minimal changes to a TypeScript file in order to run it as FlowScript.
If it isn't specified, it should be just like in TypeScript!

## Reactivity
### No need for refs, useState or effects wrappers!
When you want reactivity in React, Vue, or other popular JavaScript Frameworks, you need to use a wrapper of some sort.
This forces you to write more code and thus makes for inconveniences.
What in React you would have to write like this:
```JavaScript
//To define x
const [x, setX] = useState(1);
//To change x
setX(2);
```
...can be done like this in FlowScript:
```JavaScript
//To define x
let x = 1;
//To change x
x = 2;
```
FlowScript enables this by handling all reactivity under the hood with its internal reactivity engine written in Rust!

### A new keyword: 'rel'
'let'-reactivity is great, but sometimes you don't want public reactivity. To keep the reactivity private when that is called for, FlowScript adds a middle-ground between 'const' and 'let': 'rel'.
It is short for 'relationship', and denotes a constant relationship that is reactive. It is loosely inspired by Vue's computed()-values.
This is how 'rel' works:

```JavaScript
//To define y, assuming x from previous examples
rel y = 1 + x;
//To change y, you must change x!
//WILL NOT WORK: y = 3 + x;
x = 3; //Changing x makes FlowScript recalculate y. New value = 6
```

### Reactivity is opt-in!
You don't always want reactivity. FlowScript seeks to make it intuitive what is and is not reactive. When you have a "constant", do you expect it to be reactive? Of course not! Hence, 'const' excepts whatever you define with it from the reactivity engine.
It is currently considered a good practice in Type/JavaScript to use 'const' for all cases where you don't intend for the value to change (and conversely, 'let' when you expect changes), and this taps into that.
This keeps it intuitive, makes it easy to refactor from Type/JavaScript to FlowScript incrementally, and saves the reactivity engine from unnecessary work.
As in Type/JavaScript, when you define an object with 'const', you can still change the values of its keys. If you (for some reason) need a value to not be reactive but still changeable, just wrap it in an object:

```JavaScript
//As before, assuming values from previous examples
const z = { value: x}; //Since x = 3, logged value is 3
//To change
//WILL NOT WORK: x = 5; Since z is a constant it remembers its value and not its source, and is still 3.
z.value = 6; //This will work, and will not trigger any reactivity.
```

### Reactive Functions
FlowScript introduces two patterns for creating reactive functions: The effect-pattern, and the 'rel'-pattern.

#### The effect-pattern
Based on React's useEffect and Vue's watchEffect, the effect pattern interacts with variables in the same scope. It executes immediately where it is defined, and again if any reactive variable it accesses changes.
To make a FlowScript effect function, you define a function in any way you like ('const' will also work!), but:
1. It must not take any arguments.
2. It must not specify any return.
3. It must not be exported.

If these criteria are met, it will be treated as an effect:
```JavaScript
// To define a reactive effect-function, assuming the values from previous examples
function myEffect() {
  console.log(y); //Logs the value of y, which is currently 6
  z.value = y;
}
//From here on it will run whenever y changes
x = 4 //This triggers recalculation of y
// That in turn triggers myEffect,
// logging y's value (now: 7) and changing the value of z to 7
```

Are you beginning to see why this is *flow*-script?
Effects can also be called manually if you really want to.
They do some extra things under the hood that you should know about.

##### Implicit if-check
Effect functions expect two primary use cases, and therefore include an implicit if-check.
This check is very simple: "if all accessed variables are truthy, then execute", which can be useful when working with async (it allows FlowScript to work without considering Promises) or for simple effects.
This implicit check gets removed if the top level of the effect is an if-check. Then your check gets used instead. This lets you engineer very specific effects when you need them.
If you provide your own if-check, take note: It should not include an else-segment (but else-if-segments are OK!).
These if-checks get used in logging to provide information on whether the effect did anything or not when it triggered.
Compared to React's useEffect, these if-checks provide a more predictable format for specifying both dependencies and their relevant states in one operation.
Here is an example:

```JavaScript
//As before, assuming values from previous examples
const myFancyEffect = () => {
  if (y > 5) { //y is accessed, so it is a dependency
    x = 1; //x is only changed, not accessed, so it is not a dependency
    //implicit (default disabled) either log or return
    //console.log(`${myFancyEffect.name}`, true);
    //return true;
  }
  //implicit (default disabled) either log or return
  //console.log(`${myFancyEffect.name}`,false);
  //return false;
}
//This effect keeps y small by returning x's value to 1.
//This once again makes y = 4, which means that we log y and set z.value = 4 with the other effect...
//Since changing x changes y and myFancyEffect depends on y, it gets triggered a second time.
//This time it does nothing since y is not more than 5.
//The returns get enabled if you define the effect as async and may come in handy if you ever need to await it.
//The logs get enabled by the reactive logging directive.
```

Again: Do you see why this is *flow*-script?

#### The 'rel'-pattern
The effect-pattern can't handle arguments, because it has no way to know what the arguments would be.
Using the 'rel'-keywordlet's you specify arguments for any normal function to make it reactive! ('let' works too, but is a bit messier since that implies you will reassign the function in the variable later!)
You can think of any variable defined with 'rel' as follows:
```JavaScript
//Remember when we defined rel y = 3 + x?
//The following is basically equivalent in more traditional code:
rel y = () => {
  return 3 + x;
}
//It runs whenever the accessed variables change
```
Consequently, if you want a function to run reactively but the function requires arguments, you can arrange that like this:
```JavaScript
// As before, assuming values from previous examples
rel recurringFunction = anyNormalFunction(x, z);
//We specify the arguments, and 'rel' handles the rest!
//Now anyNormalFunction will run with the reactive variable x as an argument.
//It will always combine it with the normal object z.
//You can probably find a use for this!
```

### Seamless async, with no *need* for Promises
When a 'let' or 'rel' variable receives a return from an async function, it is *not yet defined before the return is ready*, but *starts as **undefined***.
This means that it triggers effects that depend on it right away, but if you use the default implicit if-check, the effects cancel.
Until, that is, the **promise is fulfilled**, at which point the Promise<any | Whatever> can be handled as just any | Whatever, and you can proceed to use it as intended.
Hence, there is no *need* to wait for Promises, but it may still be worth using them and await, since from the moment you start using effects for this you're forced to keep using effects for everything that builds on it, which can easily get out of hand.
Example:
```TypeScript
const itemHolder = { value: string | undefined };
//Including types on someAsyncFunction to show what you get to skip
rel resolvedPromise = someAsyncFunction<Promise<string>>(someArgument);
// promiseUser does nothing before 
const promiseUser = () => {
  console.log(typeof resolvedPromise); //only ever logs 'string'
  itemHolder.value = resolvedPromise;
}
```

### Reactive logging with **&log;**
Keeping track of values in a reactive flow can be very difficult while debugging.
Anticipating this problem, FlowScript includes the **&log;** directive.
Append any line that defines something with **&log;**, and FlowScript will note how it got to the line (the immediate parent function - this may be valuable if the file gets invoked in several places, as a React component might be!), the name of the logged variable or function, and its current value if it was a variable (this includes 'rel'-pattern reactive functions).
It will then log every time the variable changes, or for effects, true if it executed and false if it did not, and include the additional information it noted about it.
This should make it a lot easier to track the changes to a reactive variable and when/how much a reactive function triggers.
Example:
```JavaScript
let reactiveString = "Hi, I'm reactive!"&log;
```
### Reactivity Best Practices
1. Don't use 'let' if you don't need to! Just as you should use 'const' when you don't need 'let' in Type/JavaScript, you should use 'rel' when you don't need 'let'. It makes it harder to make mistakes!
   As a rule of thumb: 'let' ***lets you start*** reactivity, and 'rel' ***continues*** reactivity!
2. Prefix reactive variables and functions with 'r'! It can be easy to forget what is and is not reactive, and a prefix makes it much easier to tell at a glance!
   ```JavaScript
   //Instead of let x = "something":
   let rx = "something";
   ```
3. Don't use reactivity if you don't need it! Seriously. Reactivity is a lot of overhead, which you don't want to pick up when it isn't necessary.

## Why mimic TypeScript?
Since FlowScript mimics but is not built on Type/JavaScript, it does not need to inherit everything from them.
Here are some benefits with starting from scratch.

### Consolidating '**null**' into '**undefined**'
The current plan is to treat every instance of '**null**' as though it were '**undefined**'.
Since there are only very small differences between these two and those differences are mostly semantic, this *should* work just fine.
Consolidating '**null**' into '**undefined** rather than vice versa sounded better because it is a bit more practical; You can do everything '**null**' let's you do with '**undefined**', and a little more to boot!
Consider this:
```TypeScript
type MyType = {
  name: string;
  directive?: true;
}
```
If this type is used as the accepted argument of a function, it accepts but does not request the directive, in a simple syntax that has no equivalent for '**null**'.
This is yet another way that FlowScript reduces the amount of code you need to write.

### Types from Python
In addition to the arrays and objects of Type/JavaScript, FlowScript takes a cue from Python's array types by adding more direct tuple and set support.

#### Tuples
As in Python, Tuples are ordered (so you can access them by index) and **unchangeable** (so there is *never* a point in defining them with 'let' or 'rel') outside of deletion, and can have duplicate values, as well as different datatypes.
To use Tuples in FlowScript, use the following syntax:

```TypeScript
//To specify Tuple type, specify for every index.
//Type inference will identify the types automatically.
//Only specify type if it will be reused!
type MyTuple: (string, number, boolean);
//To define tuple (with type), do as follows:
const myTuple: MyTuple = ("Warhammer", 40000, false);
//Tuples can be named
//Type spefication for named tuple:
type MyNamedTuple: (name: string, age: number, married: boolean);
//If names are in type, you can skip names in definition, but you can't skip order!
const myNamedTuple: MyNamedTuple = ("Elizabeth", 20, false);
//To access tuple, use index. You can use name if the tuple is named:
console.log(myTuple[0]);
console.log(myNamedTuple[1]);
console.log(myNamedTuple[name]);
```

#### Sets
As in Python, sets are unordered (so you can not access them by index), and their *items* are **unchangeable**. You can add or remove items, but not change them.
Thus, they have some interactions with FlowScript reactivity.
Sets can not contain duplicate values, but can contain different datatypes.
Since they are unordered, you specify types in the same way as you would with a normal TypeScript Array, and the Set only accepts members of the specified type(s).
```TypeScript
type StringSet string{};
type MultiTypedSet (string | boolean | number){};
```
Sets are useful for checking membership quickly and so on. They have relatively few parameters that are useful in a reactive context.
If you have a reactive tuple, and a reactive variable depends on it (for example: rel rBorgInSet = rNameSet.has("Borg);), adding an element to the set tells the variable that it has to run the operation again, changing the value of the variable accordingly.
