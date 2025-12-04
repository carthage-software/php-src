--TEST--
Type alias autoloading with type_alias_exists
--FILE--
<?php

spl_autoload_register(function ($name) {
    echo "Autoloading: $name\n";
    if ($name === 'MyTypeAlias') {
        eval('type MyTypeAlias = int;');
    }
});

var_dump(type_alias_exists('MyTypeAlias', false));
var_dump(type_alias_exists('MyTypeAlias', true));
var_dump(type_alias_exists('MyTypeAlias', false));

?>
--EXPECT--
bool(false)
Autoloading: MyTypeAlias
bool(true)
bool(true)
