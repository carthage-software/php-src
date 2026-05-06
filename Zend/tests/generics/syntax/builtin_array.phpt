--TEST--
Generic syntax: array<...> with various arities
--FILE--
<?php
function f1(array<int> $x): array<string> { return []; }
function f2(array<string, int> $x): array<int, string> { return []; }
function f3(array<int, string, float> $x): void {}

foreach (['f1', 'f2', 'f3'] as $name) {
    $r = new ReflectionFunction($name);
    $pt = $r->getParameters()[0]->getType();
    echo "$name: ", $pt->getName(), " args=", count($pt->getGenericArguments()), "\n";
}
?>
--EXPECT--
f1: array args=1
f2: array args=2
f3: array args=3
