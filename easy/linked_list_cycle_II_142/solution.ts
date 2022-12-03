class ListNode {
    val: number
    next: ListNode | null
    constructor(val?: number, next?: ListNode | null) {
	this.val = (val===undefined ? 0 : val)
	this.next = (next===undefined ? null : next)
    }
}


function detectCycle(head: ListNode | null): ListNode | null {
    if(head === null || head.next === null) {
	return null
    }

    let fast = head.next
    let slow = head


    while(fast !== slow) {
	if(fast === null || fast.next === null) {
	    return null
	} else {
	    fast = fast.next.next
	    slow = slow.next
	}
    }

    fast = fast.next
    while(head !== fast) {
	head = head.next
	fast = fast.next
    }

    return head
}
