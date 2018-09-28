package apicaller

import "testing"

func TestCallApi(t *testing.T) {
	//Arrange

	//Act
	apiResult, err := CallAPI("https://jsonplaceholder.typicode.com/posts/1")

	//Assert
	if err != nil || apiResult == "" {
		t.Fatal("API Caller is not working")
	}
}
