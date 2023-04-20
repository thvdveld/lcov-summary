use nom::bytes::complete::{tag, take_until1};
use nom::IResult;

pub fn source_file_path(input: &str) -> IResult<&str, &str> {
    let (file_path, _) = tag("SF:")(input)?;
    Ok(("", file_path))
}

pub fn function_name(input: &str) -> IResult<&str, (usize, &str)> {
    let (input, _) = tag("FN:")(input)?;
    let (input, line_number) = take_until1(",")(input)?;
    let (name, _) = tag(",")(input)?;

    Ok(("", (line_number.parse::<usize>().unwrap(), name)))
}

pub fn function_hit_count(input: &str) -> IResult<&str, (usize, &str)> {
    let (input, _) = tag("FNDA:")(input)?;
    let (input, line_number) = take_until1(",")(input)?;
    let (name, _) = tag(",")(input)?;

    Ok(("", (line_number.parse::<usize>().unwrap(), name)))
}

fn tag_number<'i>(input: &'i str, t: &'_ str) -> IResult<&'i str, usize> {
    let (found, _) = tag(t)(input)?;
    Ok(("", found.parse::<usize>().unwrap()))
}

pub fn functions_found(input: &str) -> IResult<&str, usize> {
    tag_number(input, "FNF:")
}

pub fn functions_hit(input: &str) -> IResult<&str, usize> {
    tag_number(input, "FNH:")
}

//pub fn line_number_hit_count(input: &str) -> IResult<&str, (usize, usize)> {
//let (input, _) = tag("DA:")(input)?;
//let (input, line_number) = take_until1(",")(input)?;
//let (hit_count, _) = tag(",")(input)?;

//Ok((
//"",
//(
//line_number.parse::<usize>().unwrap(),
//hit_count.parse::<usize>().unwrap(),
//),
//))
//}

pub fn lines_found(input: &str) -> IResult<&str, usize> {
    tag_number(input, "LF:")
}

pub fn lines_hit(input: &str) -> IResult<&str, usize> {
    tag_number(input, "LH:")
}

pub fn branches_found(input: &str) -> IResult<&str, usize> {
    tag_number(input, "BRF:")
}

pub fn branches_hit(input: &str) -> IResult<&str, usize> {
    tag_number(input, "BRH:")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_file_path() {
        let input = "SF:/home/thvdveld/source/vub/smoltcp/src/iface/fragmentation.rs";

        let (_, file_name) = source_file_path(input).unwrap();

        assert_eq!(
            file_name,
            "/home/thvdveld/source/vub/smoltcp/src/iface/fragmentation.rs"
        );
    }

    #[test]
    fn test_function_name() {
        let input = "FN:110,_RINvMs2_NtNtCshpVWEOJQZRA_7smoltcp5iface13fragmentationINtB6_15PacketAssemblerpE8add_withpEBa_";

        let (_, (line_number, name)) = function_name(input).unwrap();

        assert_eq!(line_number, 110);
        assert_eq!(
            name,
            "_RINvMs2_NtNtCshpVWEOJQZRA_7smoltcp5iface13fragmentationINtB6_15PacketAssemblerpE8add_withpEBa_"
        );
    }

    #[test]
    fn test_function_hit_count() {
        let input = "FNDA:0,_RINvMs2_NtNtCshpVWEOJQZRA_7smoltcp5iface13fragmentationINtB6_15PacketAssemblerpE8add_withpEBa_";

        let (_, (hits, name)) = function_hit_count(input).unwrap();

        assert_eq!(hits, 0);
        assert_eq!(
            name,
            "_RINvMs2_NtNtCshpVWEOJQZRA_7smoltcp5iface13fragmentationINtB6_15PacketAssemblerpE8add_withpEBa_"
        );
    }

    #[test]
    fn test_functions_found() {
        let input = "FNF:38";
        let (_, found) = functions_found(input).unwrap();
        assert_eq!(found, 38);
    }

    #[test]
    fn test_functions_hit() {
        let input = "FNH:23";
        let (_, found) = functions_hit(input).unwrap();
        assert_eq!(found, 23);
    }

    #[test]
    fn test_line_number_hit_count() {
        let input = "DA:17,0";
        let (_, (line, hit)) = line_number_hit_count(input).unwrap();
        assert_eq!(line, 17);
        assert_eq!(hit, 0);
    }

    #[test]
    fn test_branches_found() {
        let input = "BRF:0";
        let (_, branches) = branches_found(input).unwrap();
        assert_eq!(branches, 0);
    }

    #[test]
    fn test_branches_hit() {
        let input = "BRH:0";
        let (_, branches) = branches_hit(input).unwrap();
        assert_eq!(branches, 0);
    }
}
