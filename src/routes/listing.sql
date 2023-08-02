select beat_map.date_ranked,
  category.category_display_name,
  song.song_name,
  song.level_author_name,
  beat_map.complexity,
  song.beat_saver_key
from beat_map
  left join category on beat_map.category_id = category.id
  left join song on song.song_hash = beat_map.song
where date_ranked >= ?
  and date_ranked <= ?
