# Jobs

Jobs are the backbone of the operation of civilian creeps. They are stored on the heap in a hashmap keyed by the Room.

Jobs update as they are picked up by the creep, for example a job associated with picking up energy off the ground will have its energy updated
when a creep selects that job, so when another creep picks up a job they are not both going for the same limited pile of energy.

## Usage

TODO
