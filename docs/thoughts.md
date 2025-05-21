Consider adding creep options to handle creeps are a more granular level based on room state.
An example from our previous code base was having an option in the creep's memory that allowed them to
get energy from links, storage, containers, etc. This let us turn off and on certain features of creeps depending on the state of affairs.

This would come very in handy for boostraping carriers. Rather than having to create a whole new role that can mine for energy,
we could simply flip on the "harvestSource" option on the carriers that are spawned and they would be able to quickly kickstart the room
