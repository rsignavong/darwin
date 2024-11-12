
    select
      recursive_filter_event_executes(
        list [ 'a', 'b', 'c' ],
        list [ '{"id": "a", "next": ["b", "d"] }' :: jsonb ] 
      );
