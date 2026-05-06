--TEST--
Reflection: type-parameter references inside extends/implements args resolve correctly
--FILE--
<?php
interface Container<X> {}
class Holder<T : object> implements Container<T> {}

$rc = new ReflectionClass('Holder');
$args = $rc->getGenericArgumentsForParentInterface('Container');

echo "count: ", count($args), "\n";
echo "class: ", get_class($args[0]), "\n";
echo "name: ", $args[0]->getName(), "\n";

$param = $args[0]->getTypeParameter();
echo "type param: ", $param->getName(), "\n";
echo "declaring: ", $param->getDeclaringEntity()->getName(), "\n";
?>
--EXPECT--
count: 1
class: ReflectionTypeParameterReference
name: T
type param: T
declaring: Holder
