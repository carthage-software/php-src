--TEST--
Reflection: classes with no generic content return null from all three accessors
--FILE--
<?php
class Plain {}
class WithParent extends Plain {}
interface IPlain {}
class WithIface implements IPlain {}
trait TPlain { public function noop(): void {} }
class WithTrait { use TPlain; }

foreach (['Plain', 'WithParent', 'WithIface', 'WithTrait'] as $cls) {
    $rc = new ReflectionClass($cls);
    $p = $rc->getGenericArgumentsForParentClass();
    $i = $rc->getGenericArgumentsForParentInterface('IPlain');
    $t = $rc->getGenericArgumentsForUsedTrait('TPlain');
    echo $cls, ": parent=", ($p === null ? "null" : "non-null"),
         " iface=", ($i === null ? "null" : "non-null"),
         " trait=", ($t === null ? "null" : "non-null"), "\n";
}
?>
--EXPECT--
Plain: parent=null iface=null trait=null
WithParent: parent=null iface=null trait=null
WithIface: parent=null iface=null trait=null
WithTrait: parent=null iface=null trait=null
