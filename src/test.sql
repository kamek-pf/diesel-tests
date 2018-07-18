select
    id,
    email,
    marketing,
    gender
from accounts
where marketing = $1
