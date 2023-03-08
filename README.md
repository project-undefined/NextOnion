# daoDTP
Implementation of the doadtp protocol, a new innovation by Project Undefined.
Decentralized, Anonymous, and onion-less, data transport protocol. 
Used for the Undedfined Messaging app. 

# Status
Right now, have just implemented simple MQTT with the iota.rs library provided by IOTA.
Other than that, the protocol has not been developed program-wise. 
It's a pretty simple protocol, so it shouldn't take an obsessivly long time to make (probably ;)).

# Notice
There is no guarentee that this protocol will acctually work, it is a relitively new idea. 

# LICENSING NOTICE
This project is licensed under the Apache-2.0-NoHarm License, found in License.md

# HOW IT WORKS
VISUAL COMING SOON. 

In short:

The client machine handles two-three threads, each a seperate connection to the IOTA network (connected to seperate nodes).

One thread is what I call the observer, and the other the sender. 

The observer connects via MQTT (TCP) to an iota node, and it's IP address can be seen.

On the other hand, the sender can only send messages, but cannot receive anything (and I mean ANYTHING). 

The sender sends messages to an IOTA network through a proxy that replaces its IP in the header of the IP sent IP packet. 

Well, since the node has no idea where the real IP packet packet to, and ACK response cannot be recieved by the sender. 

Therefore, the node does not know who acctually sent the message, but the sender does not know if the node even recieved or accepted the message.
 
That's were the observer comes in.

If the node accepts the sent data by the sender, the message will be sent by the node to the entire network. 

In this case, the observer monitors all of the events on the network. 

So, if the data was successfully sent, the observer will recieve it. 

And if the server recieves it within the next 1-2 milestones on the network, then it will confirm with the sender.

If it does not, then it will tell the sender to again and repeat this until the data is successfully sent. 

The result is, no node knows who acctually sent the origional message. 

The only possible metadata that can be collected is when the observer connects to the network, which is practically useless because all it does is read 
all events/messages from the network. 

The node the observer is connected to does not know which data the observer is acctually using, so it is not identifiable. 

Plus, the observer can be masked as a regular spammer, further helping it stay anonymous. 

The observer is also in charge of reading incoming messages from other users. 

(NOTE: this will be explained in more depth later on, but for now, this is just the general framework. )


