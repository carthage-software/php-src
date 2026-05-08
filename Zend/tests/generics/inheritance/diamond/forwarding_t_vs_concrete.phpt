--TEST--
Diamond: generic class forwarding T conflicts with a concrete arg on another path
--FILE--
<?php
interface Box<T> {}
interface Wrapper<U> extends Box<U> {}

class C<T> implements Wrapper<T>, Box<int> {}
?>
--EXPECTF--
Fatal error: C inherits Box<T> via Wrapper and Box<int> via Box in %s on line %d
