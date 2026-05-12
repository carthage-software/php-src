--TEST--
Reflection: getGenericArgumentsForParentInterface composes substitutions through transitive paths and returns first-match for diamonds
--FILE--
<?php

interface Root<T> {}
interface Mid<X> extends Root<X> {}
interface DeepRoot<A> {}
interface DeepMid<B> extends DeepRoot<B> {}
interface DeepLeaf<C> extends DeepMid<C> {}

class FwdGeneric<U> implements Mid<U> {}

class ConcreteThroughGeneric implements Mid<int> {}

class DeepFwd<U> implements DeepLeaf<U> {}

class WithIfaceArgs<U> implements Root<U> {}
class ChildOfBound extends WithIfaceArgs<int> {}
class GrandchildOfBound extends ChildOfBound {}

class ForwardingChild<V> extends WithIfaceArgs<V> {}

class BaseViaMid implements Mid<string> {}
class ChildViaMid extends BaseViaMid {}

interface Leaf extends Mid<bool> {}

interface DA<+T> {}
interface I1 extends DA<int> {}
interface I2 extends DA<string> {}
class Diamond implements I1, I2 {}

function show(string $cls, string $ancestor): void {
    $args = (new ReflectionClass($cls))->getGenericArgumentsForParentInterface($ancestor);
    $rendered = array_map(static fn(ReflectionType $t): string => (string) $t, $args);
    printf("%-30s -> %-10s = [%s]\n", $cls, $ancestor, implode(', ', $rendered));
}

show('FwdGeneric', 'Root');
show('FwdGeneric', 'Mid');
show('ConcreteThroughGeneric', 'Root');
show('DeepFwd', 'DeepRoot');
show('DeepFwd', 'DeepMid');
show('DeepFwd', 'DeepLeaf');
show('ChildOfBound', 'Root');
show('GrandchildOfBound', 'Root');
show('ForwardingChild', 'Root');
show('ChildViaMid', 'Root');
show('ChildViaMid', 'Mid');
show('Leaf', 'Root');
show('Diamond', 'DA');

?>
--EXPECT--
FwdGeneric                     -> Root       = [U]
FwdGeneric                     -> Mid        = [U]
ConcreteThroughGeneric         -> Root       = [int]
DeepFwd                        -> DeepRoot   = [U]
DeepFwd                        -> DeepMid    = [U]
DeepFwd                        -> DeepLeaf   = [U]
ChildOfBound                   -> Root       = [int]
GrandchildOfBound              -> Root       = [int]
ForwardingChild                -> Root       = [V]
ChildViaMid                    -> Root       = [string]
ChildViaMid                    -> Mid        = [string]
Leaf                           -> Root       = [bool]
Diamond                        -> DA         = [int]
