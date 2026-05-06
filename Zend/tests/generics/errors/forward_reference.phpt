--TEST--
Errors: forward reference inside type parameter list
--FILE--
<?php
function f<U : T, T>(): void {}
?>
--EXPECTF--
Fatal error: Type parameter T referenced before declaration in %s on line %d
