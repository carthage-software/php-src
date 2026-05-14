--TEST--
Reflection: transitive parent interface arguments substitute nested type parameters
--FILE--
<?php
class Box<T> {}
interface Root<T> {}
interface PairRoot<A, B> {}
interface Mid<X> extends Root<Box<X>> {}
interface Flip<A, B> extends PairRoot<Box<B>, Box<A>> {}

class Concrete implements Mid<int> {}
class Forwarded<U> implements Mid<U> {}
class Reordered<X, Y> implements Flip<X, Y> {}

function show(string $class, string $interface): void {
    $args = (new ReflectionClass($class))->getGenericArgumentsForParentInterface($interface);
    echo "$class/$interface\n";
    foreach ($args as $arg) {
        echo "  ", $arg->getName();
        if ($arg instanceof ReflectionNamedType && $arg->hasGenericArguments()) {
            echo "<", implode(", ", array_map(
                static fn(ReflectionType $type): string => $type->getName(),
                $arg->getGenericArguments(),
            )), ">";
        }
        echo "\n";
    }
}

show(Concrete::class, Root::class);
show(Forwarded::class, Root::class);
show(Reordered::class, PairRoot::class);

$forwardedArgs = (new ReflectionClass(Forwarded::class))->getGenericArgumentsForParentInterface(Root::class);
$forwardedInner = $forwardedArgs[0]->getGenericArguments()[0];
echo "Forwarded nested parameter owner: ",
    $forwardedInner->getTypeParameter()->getDeclaringEntity()->getName(), "\n";
?>
--EXPECT--
Concrete/Root
  Box<int>
Forwarded/Root
  Box<U>
Reordered/PairRoot
  Box<Y>
  Box<X>
Forwarded nested parameter owner: Forwarded
