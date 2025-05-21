import sys

def main():
    if len(sys.argv) < 2:
        print("Use: "  + sys.argv[0] + " [Options] URL")
        print("  -X, --request <method>    Specify the HTTP request method (GET, POST, etc.)")
        print("  -H, --header <header>     Add a custom header to the request")
        print("  -d, --data <data>        Send data with the request")
        print("  -v, --verbose             Show verbose output")
        print("  -o, --output <file>    Save the response to a file")
        print("  -t, --timeout <seconds>  Define a timeout for the request")

        sys.exit(1)


if __name__ == '__main__':
    main()