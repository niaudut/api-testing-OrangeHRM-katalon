<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET_Employee_Search</name>
   <tag></tag>
   <elementGuidId>a9b258f6-2aed-48ec-90c1-96e900c5e9c8</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${token}</value>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${url}/api/v1/employee/search</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>89ab6a0b-c49c-47e6-ad31-ebda0b6450bb</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>337d9195-e2b6-4368-aea4-91aa7ea5f4a7</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

WS.verifyElementPropertyValue(response, 'data[0].firstName', &quot;Sherina&quot;)
WS.verifyElementPropertyValue(response, 'data[0].middleName', &quot;Melodi&quot;)
WS.verifyElementPropertyValue(response, 'data[0].lastName', &quot;Darmawan&quot;)
WS.verifyElementPropertyValue(response, 'data[0].code', &quot;0009&quot;)
WS.verifyElementPropertyValue(response, 'data[0].employeeId', &quot;4&quot;)
WS.verifyElementPropertyValue(response, 'data[0].fullName', &quot;Sherina Melodi Darmawan&quot;)

WS.verifyElementPropertyValue(response, 'data[1].firstName', &quot;Sherina&quot;)
WS.verifyElementPropertyValue(response, 'data[1].lastName', &quot;Darmawan&quot;)
WS.verifyElementPropertyValue(response, 'data[1].code', &quot;0010&quot;)
WS.verifyElementPropertyValue(response, 'data[1].employeeId', &quot;13&quot;)
WS.verifyElementPropertyValue(response, 'data[1].fullName', &quot;Sherina Darmawan&quot;)

WS.verifyElementPropertyValue(response, 'data[2].firstName', &quot;john&quot;)
WS.verifyElementPropertyValue(response, 'data[2].lastName', &quot;doe&quot;)
WS.verifyElementPropertyValue(response, 'data[2].code', &quot;john&quot;)
WS.verifyElementPropertyValue(response, 'data[2].employeeId', &quot;18&quot;)
WS.verifyElementPropertyValue(response, 'data[2].fullName', &quot;john doe&quot;)

//WS.verifyElementPropertyValue(response, 'data[20].firstName', &quot;Febri&quot;)
//WS.verifyElementPropertyValue(response, 'data[20].middleName', &quot;Test&quot;)
//WS.verifyElementPropertyValue(response, 'data[20].lastName', &quot;Test&quot;)
//WS.verifyElementPropertyValue(response, 'data[20].code', &quot;02&quot;)
//WS.verifyElementPropertyValue(response, 'data[20].employeeId', &quot;22&quot;)
//WS.verifyElementPropertyValue(response, 'data[20].fullName', &quot;Febri Test Test&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
