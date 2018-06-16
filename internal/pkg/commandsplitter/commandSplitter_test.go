package commandsplitter

import (
	"strings"
	"testing"
)

func TestSplitCommandQuery(t *testing.T) {
	//Arrange
	commandToSplit := "!jisho programming language"

	//Act
	splitCommand := SplitCommand(commandToSplit)

	//Assert
	if len(splitCommand) != 2 {
		t.Fatal("Split command should only return an array of size 2")
	}
	if !strings.HasPrefix(splitCommand[0], "!") {
		t.Fatal("Command should have a prefix of '!'")
	}
	if splitCommand[0] == "" || splitCommand[1] == "" {
		t.Fatal("command not parsed correctly.")
	}
}

func TestSplitCommandNoQuery(t *testing.T) {
	//Arrange
	commandToSplit := "!ping"

	//Act
	splitCommand := SplitCommand(commandToSplit)

	//Assert
	if !strings.HasPrefix(splitCommand[0], "!") {
		t.Fatal("Command should have a prefix of '!'")
	}
	if splitCommand[0] == "" || splitCommand[1] != "" {
		t.Fatal("command not parsed correctly.")
	}
}
