select difficulty,
  song,
  song_name
from beat_map
  join song on song.song_hash = beat_map.song
where date_ranked >= ?
  and date_ranked <= ?
