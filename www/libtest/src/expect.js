import { log } from './logging'

class ExpectError extends Error{
}

export async function expectStringEqual(actual, expected, message = ''){
  let actualValue;
  if ( 'function' === typeof actual ){
    try{
      const actualFunction = actual;
      actualValue = await Promise.resolve(actualFunction());
      _expectStringEqual(actualValue, expected, message);
    }
    catch(e){
      if ( e instanceof ExpectError){
        log(e);
      }
      else{
        throw e;
      }
    }
  }
  else{
    actualValue = actual;
    _expectStringEqual(actualValue, expected, message);
  }

  
  
  function _expectStringEqual(actual, expected, message){
    if ( actual !== expected){
      throw new ExpectError(
`Expected strings to equal {
        expected: '${expected}'
        actual:   '${actual}'
} --> Strings mismatch ${message}.
  
      `)
    }
  }
}

