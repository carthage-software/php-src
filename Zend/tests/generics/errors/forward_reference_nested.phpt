--TEST--
Errors: forward reference detected through nested type argument
--FILE--
<?php
class Box<X> {}
function g<U : Box<T>, T>(): void {}
?>
--EXPECTF--
Fatal error: Type parameter T referenced before declaration in %s on line %d
