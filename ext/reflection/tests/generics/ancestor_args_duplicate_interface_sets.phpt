--TEST--
Reflection: duplicate generic interface bindings expose every argument set
--FILE--
<?php
interface Foo<T = mixed> {}
interface Root<T> {}
interface Mid<X> extends Root<X> {}
interface DA<+T> {}
interface I1 extends DA<int> {}
interface I2 extends DA<string> {}

class Direct implements Foo<string>, Foo<int> {}
class ThroughMid implements Mid<string>, Mid<int> {}
class WithoutArgs implements Foo {}
class ParentDirect implements Foo<bool>, Foo<float> {}
class ChildDirect extends ParentDirect {}
class ParentGeneric<T> implements Root<T> {}
class ChildGeneric<U> extends ParentGeneric<U> {}
class Diamond implements I1, I2 {}

function render_sets(string $class, string $interface): void {
    $sets = (new ReflectionClass($class))->getGenericArgumentSetsForParentInterface($interface);
    echo "$class/$interface\n";
    foreach ($sets as $set) {
        echo "  [", implode(", ", array_map(
            static fn(ReflectionType $type): string => $type->getName(),
            $set,
        )), "]\n";
    }
}

render_sets(Direct::class, Foo::class);
render_sets(ThroughMid::class, Root::class);
render_sets(WithoutArgs::class, Foo::class);
render_sets(ChildDirect::class, Foo::class);
render_sets(ChildGeneric::class, Root::class);
render_sets(Diamond::class, DA::class);

try {
    (new ReflectionClass(Direct::class))->getGenericArgumentSetsForParentInterface(Root::class);
} catch (ReflectionException $e) {
    echo $e->getMessage(), "\n";
}
?>
--EXPECT--
Direct/Foo
  [string]
  [int]
ThroughMid/Root
  [string]
  [int]
WithoutArgs/Foo
  []
ChildDirect/Foo
  [bool]
  [float]
ChildGeneric/Root
  [U]
Diamond/DA
  [int]
  [string]
Root is not an ancestor interface of Direct
