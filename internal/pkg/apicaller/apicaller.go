package apicaller

import (
	"errors"
	"io/ioutil"
	"net/http"
)

/*CallAPI :
 * This function is separated from the other modules so that we have the freedom
 *  to change what backend we're using our HTTP library.
 * Expects a fully qualified URI as an argument and will return the body of the
 *  response and/or an error
 */
func CallAPI(s string) (string, error) {
	resp, err := http.Get(s)
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return "", errors.New("http status code, " + string(resp.StatusCode))
	}

	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		return string(body), err
	}

	return string(body), nil
}
