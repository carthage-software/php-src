--TEST--
Type alias conflicts with class
--FILE--
<?php

class Foo {}

type Foo = int;

?>
--EXPECTF--
Fatal error: Cannot redeclare Foo (already defined) in %s on line %d
