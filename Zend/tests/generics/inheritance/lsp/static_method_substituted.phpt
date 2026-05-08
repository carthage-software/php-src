--TEST--
Parametric LSP: static methods on the parent are also substituted in the child
--FILE--
<?php
abstract class Factory<T> {
    abstract public static function make(): T;
}

class IntFactory extends Factory<int> {
    public static function make(): int { return 7; }
}

echo IntFactory::make(), "\n";
?>
--EXPECT--
7
