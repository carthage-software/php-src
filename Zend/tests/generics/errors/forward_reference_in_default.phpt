--TEST--
Errors: forward reference in default of earlier parameter
--FILE--
<?php
class C<U = T, T> {}
?>
--EXPECTF--
Fatal error: Type parameter T referenced before declaration in %s on line %d
