--TEST--
Type alias with namespaces
--FILE--
<?php

namespace App\Types {
    type Username = string;
}

namespace App\Services {
    use App\Types\Username;

    function greet(Username $name): void {
        echo "Hello, $name!\n";
    }
}

namespace {
    App\Services\greet("World");
}

?>
--EXPECT--
Hello, World!
