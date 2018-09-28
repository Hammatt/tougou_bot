package commandsplitter

import (
	"strings"
)

/*SplitCommand : takes a whole input string and returns the "command", and the "args"
 */
func SplitCommand(stringToSplit string) (string, string) {
	var splitCommand [2]string

	i := strings.Index(stringToSplit, " ")

	if i > 0 {
		splitCommand[0] = stringToSplit[:i]
		splitCommand[1] = stringToSplit[i+1:]
	} else { //case where we recieve a command without a query.
		splitCommand[0] = stringToSplit
	}

	return splitCommand[0], splitCommand[1]
}
