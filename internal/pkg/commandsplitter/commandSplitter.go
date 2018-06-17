package commandsplitter

import (
	"strings"
)

/*SplitCommand : takes a "command" string and splits it into an array of size 2 where the
 * 0th index is "just the command" and the 1st index is "the query"
 */
func SplitCommand(stringToSplit string) [2]string {
	var splitCommand [2]string

	i := strings.Index(stringToSplit, " ")

	if i > 0 {
		splitCommand[0] = stringToSplit[:i]
		splitCommand[1] = stringToSplit[i+1:]
	} else { //case where we recieve a command without a query.
		splitCommand[0] = stringToSplit
	}

	return splitCommand
}
