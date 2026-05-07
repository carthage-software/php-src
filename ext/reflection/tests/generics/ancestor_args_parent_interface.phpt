--TEST--
Reflection: getGenericArgumentsForParentInterface returns args from implements / interface-extends; throws when not ancestor
--FILE--
<?php
interface I<K = mixed, V = mixed> {}
interface J<T> {}
interface K1 extends I<int, string> {}

class WithArgs implements I<int, string> {}
class WithoutArgs implements I {}
class NotImplements {}
class Multi implements I<bool, float>, J<string> {}

function show(string $cls, string $iface): void {
    try {
        $args = (new ReflectionClass($cls))->getGenericArgumentsForParentInterface($iface);
    } catch (ReflectionException $e) {
        echo "$cls/$iface: throw ({$e->getMessage()})\n";
        return;
    }
    if (!$args) {
        echo "$cls/$iface: []\n";
        return;
    }
    echo "$cls/$iface: ", implode(",", array_map(fn($t)=>$t->getName(), $args)), "\n";
}

show('WithArgs', 'I');
show('WithArgs', 'i'); // case insensitive
show('WithoutArgs', 'I');
show('NotImplements', 'I');
show('Multi', 'I');
show('Multi', 'J');
show('K1', 'I');
?>
--EXPECT--
WithArgs/I: int,string
WithArgs/i: int,string
WithoutArgs/I: []
NotImplements/I: throw (I is not an ancestor interface of NotImplements)
Multi/I: bool,float
Multi/J: string
K1/I: int,string
