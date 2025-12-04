--TEST--
Type alias circular reference: direct self-reference
--FILE--
<?php

type A = A;

?>
--EXPECTF--
Fatal error: Type alias A cannot reference itself, either directly or through other type aliases in %s on line %d
