package commandsplitter

import (
	"strings"
	"testing"
)

func TestSplitCommandQuery(t *testing.T) {
	//Arrange
	commandToSplit := "!jisho programming language"

	//Act
	command, args := SplitCommand(commandToSplit)

	//Assert
	if !strings.HasPrefix(command, "!") {
		t.Fatal("Command should have a prefix of '!'")
	}
	if command == "" || args == "" {
		t.Fatal("command not parsed correctly.")
	}
}

func TestSplitCommandNoQuery(t *testing.T) {
	//Arrange
	commandToSplit := "!ping"

	//Act
	command, args := SplitCommand(commandToSplit)

	//Assert
	if !strings.HasPrefix(command, "!") {
		t.Fatal("Command should have a prefix of '!'")
	}
	if command == "" || args != "" {
		t.Fatal("command not parsed correctly.")
	}
}
