TO DO:

+ add persistence when finished

+ add error handling if block fails to add to node

+ consensus between nodes to prevent simultaneous node creation aka no two nodes #4 pointing to #3 for example, create "retry mechanism"
	- rudimentary consensus determines longest "most up to date chain from either local and remote and uses that"
	
+ security layer between connecting nodes, currently client request and node response are broadcast through the entire network