import attrs

from plugins.cluster.model import Cluster
from plugins.leader.leader import Leader
from plugins.leader.follower import Follower


@attrs.frozen
class LeaderFollower:
    cluster: Cluster
    current_node: str
    
    @property
    def leader(self) -> Leader:
        leader_node = max(self.cluster.nodes)
        leader_id, leader_url = leader_node.split("@")
        leader = Leader(id=leader_id, url=leader_url, followers=[])
    
        followers: list[Follower] = list(
            map(
                lambda node: Follower(
                    id=node.split("@")[0],
                    url=node.split("@")[1],
                    follows=leader
                ),
                filter(lambda node: node != leader_node, self.cluster.nodes)
            )
        )
        
        leader.followers = followers
        return leader
