--TEST--
Scoping: instanceof T uses class T, not type parameter
--FILE--
<?php
class T {}
function check<T : object>($x): bool {
    return $x instanceof T;
}
var_dump(check(new T));
var_dump(check(new stdClass));
?>
--EXPECT--
bool(true)
bool(false)
